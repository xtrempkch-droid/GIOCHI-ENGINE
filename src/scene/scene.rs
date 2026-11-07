//! Contrato de cena e contexto de recursos compartilhados.
//! Status: trait `Scene`, `SceneContext` e transições de entrada/saída implementados.

use sdl2::event::Event;

use crate::{
    assets::ResourceManager,
    audio::AudioSystem,
    input::Input,
    render::{renderer::RenderContext, RenderError},
};

/// Recursos disponíveis para uma cena durante `on_enter` e `on_exit`.
///
/// As texturas obtidas aqui são compartilhadas por `Rc`; a cena deve guardá-las
/// para desenhar nos quadros seguintes, já que `render` não recebe o contexto.
pub struct SceneContext<'s, 'w> {
    /// Cache de recursos da engine.
    pub resources: &'s mut ResourceManager<'w>,
    /// Dispositivo de áudio, quando a inicialização foi bem-sucedida.
    pub audio: Option<&'s mut AudioSystem>,
}

/// Comportamento comum a menu, jogo e telas de pausa.
///
/// O contrato é livre de objetos genéricos, portanto `Box<dyn Scene>` é válido e
/// o [`super::SceneManager`] guarda cenas heterogêneas na mesma pilha.
pub trait Scene {
    /// Prepara a cena carregando recursos e iniciando música.
    fn on_enter(&mut self, _ctx: &mut SceneContext<'_, '_>) {}
    /// Libera recursos ou para a música ao sair da cena.
    fn on_exit(&mut self) {}
    /// Atualiza a simulação em segundos com a entrada do quadro.
    fn update(&mut self, dt: f32, input: &Input);
    /// Desenha a cena e propaga falhas do backend.
    fn render(&mut self, ctx: &mut RenderContext<'_>) -> Result<(), RenderError>;
    /// Trata um evento SDL; retorna `true` quando o evento foi consumido.
    ///
    /// Eventos não consumidos sobem para as cenas abaixo na pilha.
    fn handle_event(&mut self, _event: &Event) -> bool {
        false
    }
}
