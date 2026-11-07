//! Fachada unificada da engine: plataforma, entrada, renderização, recursos,
//! áudio e pilha de cenas em uma única estrutura.
//! Status: `Engine` com ciclo de vida completo, cenas e áudio opcional implementada.
//!
//! A janela é emprestada porque o cache de texturas precisa viver junto do
//! criador SDL que a janela possui. Essa divisão é feita por
//! [`crate::platform::Window::split`], sem `unsafe` e sem auto-referência.

use crate::{
    assets::{AssetError, ResourceManager},
    audio::{AudioError, AudioSystem, MixerConfig},
    core::game_loop::{self, FIXED_DT},
    input::Input,
    platform::{PlatformError, RenderTarget, Sdl, Window},
    render::{texture::TextureLoader, RenderError},
    scene::{Scene, SceneContext, SceneManager},
};

/// Configuração da engine.
#[derive(Debug, Clone, PartialEq)]
pub struct EngineConfig {
    /// Título da janela.
    pub title: String,
    /// Largura lógica inicial da janela.
    pub width: u32,
    /// Altura lógica inicial da janela.
    pub height: u32,
    /// Abre o dispositivo de áudio durante a inicialização.
    ///
    /// Quando `false`, `Engine::audio` permanece `None` e as cenas não recebem áudio.
    pub audio: bool,
    /// Configuração do mixer, usada apenas quando `audio` é `true`.
    pub mixer: MixerConfig,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            title: "GIOCHI-ENGINE".to_owned(),
            width: 1280,
            height: 720,
            audio: true,
            mixer: MixerConfig::default(),
        }
    }
}

impl EngineConfig {
    /// Configuração com o título e as dimensões indicados.
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            title: title.into(),
            width,
            height,
            ..Self::default()
        }
    }

    /// Define se o áudio deve ser aberto.
    pub fn with_audio(mut self, audio: bool) -> Self {
        self.audio = audio;
        self
    }

    /// Define a configuração do mixer.
    pub fn with_mixer(mut self, mixer: MixerConfig) -> Self {
        self.mixer = mixer;
        self
    }
}

/// Falhas ao montar ou executar a engine.
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    /// A plataforma SDL2 não pôde ser inicializada.
    #[error(transparent)]
    Platform(#[from] PlatformError),
    /// O contexto de renderização falhou.
    #[error(transparent)]
    Render(#[from] RenderError),
    /// Um recurso não pôde ser carregado.
    #[error(transparent)]
    Asset(#[from] AssetError),
    /// O áudio não pôde ser inicializado.
    #[error(transparent)]
    Audio(#[from] AudioError),
    /// Nenhuma cena foi informada para executar.
    #[error("a engine precisa de uma cena inicial para executar")]
    MissingScene,
}

/// Fachada que reúne os subsistemas da engine em um único ciclo de vida.
///
/// Ordem de destruição (declaração dos campos): cenas e recursos são liberados
/// antes do alvo de desenho, a entrada antes do SDL2 e o áudio antes do contexto
/// raiz. Isso evita destruir texturas depois do renderer e o mixer depois do SDL.
pub struct Engine<'w> {
    config: EngineConfig,
    scenes: SceneManager<'w>,
    resources: ResourceManager<'w>,
    texture_creator: &'w sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    target: RenderTarget<'w>,
    input: Input,
    audio: Option<AudioSystem>,
    sdl: Sdl,
}

impl<'w> Engine<'w> {
    /// Monta a engine a partir de um SDL2 e uma janela já inicializados.
    ///
    /// A janela é emprestada por `'w`; mantenha-a viva durante toda a execução.
    /// Use [`crate::platform::init_sdl`] e [`Window::new`] antes de chamar este
    /// construtor, ou [`bootstrap`] para o fluxo completo.
    pub fn new(
        sdl: Sdl,
        window: &'w mut Window,
        config: EngineConfig,
    ) -> Result<Self, EngineError> {
        let (target, texture_creator) = window.split();
        // Valida o SDL_image na montagem, mas sem manter o loader: texturas são
        // vinculadas ao criador e continuam válidas após o `IMG_Quit`.
        TextureLoader::new(texture_creator)?;

        let input = Input::new(&sdl.ctx);
        let audio = if config.audio {
            match AudioSystem::init(config.mixer) {
                Ok(audio) => Some(audio),
                Err(error) => {
                    log::warn!("áudio indisponível, seguindo sem som: {error}");
                    None
                }
            }
        } else {
            None
        };

        Ok(Self {
            config,
            scenes: SceneManager::new(),
            resources: ResourceManager::new(),
            texture_creator,
            target,
            input,
            audio,
            sdl,
        })
    }

    /// Configuração em uso.
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    /// Cache de recursos compartilhados.
    pub fn resources(&self) -> &ResourceManager<'w> {
        &self.resources
    }

    /// Cache de recursos para carregar ou consultar texturas, sons e músicas.
    pub fn resources_mut(&mut self) -> &mut ResourceManager<'w> {
        &mut self.resources
    }

    /// Carrega ou reutiliza uma textura pelo caminho.
    pub fn texture(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<std::rc::Rc<crate::render::texture::Texture<'w>>, EngineError> {
        let loader = TextureLoader::new(self.texture_creator)?;
        Ok(self.resources.texture(&loader, path)?)
    }

    /// Cria uma fábrica de texturas vinculada à janela da engine.
    ///
    /// Útil para texturas procedurais: a textura resultante tem o lifetime `'w`
    /// da janela, e não o do loader temporário.
    pub fn texture_loader(&self) -> Result<TextureLoader<'w>, RenderError> {
        TextureLoader::new(self.texture_creator)
    }

    /// Pilha de cenas.
    pub fn scenes(&self) -> &SceneManager<'w> {
        &self.scenes
    }

    /// Pilha de cenas para empilhar, remover ou substituir.
    pub fn scenes_mut(&mut self) -> &mut SceneManager<'w> {
        &mut self.scenes
    }

    /// Entrada unificada do quadro.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// Dispositivo de áudio, quando disponível.
    pub fn audio(&self) -> Option<&AudioSystem> {
        self.audio.as_ref()
    }

    /// Contexto SDL2 subjacente.
    pub fn sdl(&self) -> &Sdl {
        &self.sdl
    }

    /// Executa a cena inicial até Quit/Escape ou até a pilha esvaziar.
    ///
    /// Transições pedidas por `push`, `pop` ou `replace` são aplicadas entre
    /// quadros, com `on_exit`/`on_enter` chamados na ordem correta.
    pub fn run(mut self, initial_scene: Box<dyn Scene + 'w>) -> Result<(), EngineError> {
        self.scenes.push(initial_scene);
        self.apply_transitions();

        let mut stepper = game_loop::FixedStepper::new();
        while self.poll_frame() {
            for _ in 0..stepper.advance() {
                self.scenes.update(game_loop::FIXED_DT, &self.input);
                self.apply_transitions();
            }
            self.draw_frame()?;
            self.apply_transitions();
            stepper.wait();
        }
        Ok(())
    }

    /// Executa exatamente um quadro, útil para testes headless e integração.
    ///
    /// Não dorme: o chamador controla o ritmo. Retorna `false` quando o loop
    /// deve encerrar (Quit/Escape ou pilha vazia).
    pub fn step(&mut self, dt: f32) -> Result<bool, EngineError> {
        let keep_running = self.poll_frame();
        self.scenes.update(dt.max(FIXED_DT), &self.input);
        self.draw_frame()?;
        self.apply_transitions();
        Ok(keep_running && !self.scenes.is_empty())
    }

    /// Coleta eventos, atualiza a entrada, encaminha eventos às cenas e informa
    /// se o loop deve continuar.
    pub fn poll_frame(&mut self) -> bool {
        let (keep_running, events) = self.sdl.poll_frame(&mut self.input);
        for event in &events {
            self.scenes.handle_event(event);
        }
        keep_running && !self.scenes.is_empty()
    }

    /// Lê os pixels do quadro atual em RGBA8888.
    ///
    /// Serve para capturas de tela e para verificar a renderização em testes
    /// headless ou em hardware cujo framebuffer legado não reflete o KMSDRM.
    pub fn read_pixels(&mut self, rect: crate::math::rect::Rect) -> Result<Vec<u8>, RenderError> {
        let context = self.target.context();
        context.read_pixels(rect)
    }

    /// Desenha a cena do topo e apresenta o quadro.
    pub fn draw_frame(&mut self) -> Result<(), EngineError> {
        let mut context = self.target.context();
        context.ensure_cleared(crate::math::color::Color::rgb(0, 0, 0));
        self.scenes.render(&mut context)?;
        context.present();
        Ok(())
    }

    fn apply_transitions(&mut self) {
        let mut context = SceneContext {
            resources: &mut self.resources,
            audio: self.audio.as_mut(),
        };
        self.scenes.apply_pending(&mut context);
    }
}

/// Inicializa SDL2, cria a janela e executa a cena inicial.
///
/// Cria a janela, roda o loop e libera tudo ao final. Para reutilizar a janela
/// entre execuções, use [`Engine::new`] com uma janela emprestada.
///
/// Cria a janela, roda o loop e libera tudo ao final. Para reutilizar a janela
/// entre execuções, use [`Engine::new`] com uma janela emprestada.
pub fn bootstrap(config: EngineConfig, scene: Box<dyn Scene>) -> Result<(), EngineError> {
    let sdl = crate::platform::init_sdl()?;
    let mut window = Window::new(&sdl, &config.title, config.width, config.height)?;
    let engine = Engine::new(sdl, &mut window, config)?;
    engine.run(scene)
}
