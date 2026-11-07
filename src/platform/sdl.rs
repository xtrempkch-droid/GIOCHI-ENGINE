//! Inicialização SDL2 com seleção condicional do driver de vídeo KMSDRM.
//! Status: inicialização, driver, coleta de eventos e entrada unificada implementados.

use sdl2::{event::Event, hint, keyboard::Keycode, EventPump, VideoSubsystem};

use crate::input::Input;

use super::PlatformError;

/// Estado SDL2 mantido durante a vida da aplicação.
pub struct Sdl {
    event_pump: EventPump,
    /// Subsistema de vídeo inicializado.
    pub video: VideoSubsystem,
    /// Contexto raiz SDL2.
    pub ctx: sdl2::Sdl,
}

impl Sdl {
    /// Retorna o subsistema de vídeo para criação de janelas SDL2.
    pub fn video(&self) -> &VideoSubsystem {
        &self.video
    }

    /// Coleta os eventos SDL pendentes para consumidores da camada de plataforma.
    pub fn poll_raw_events(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter().collect()
    }

    /// Coleta eventos, atualiza a entrada portátil e informa se o loop deve continuar.
    ///
    /// Retorna `false` quando a janela pede para fechar ou quando Escape é pressionado.
    pub fn poll_input(&mut self, input: &mut Input) -> bool {
        self.poll_frame(input).0
    }

    /// Coleta eventos, atualiza a entrada e devolve os eventos para as cenas.
    ///
    /// O booleano indica se o loop deve continuar; os eventos retornados já foram
    /// aplicados à entrada e podem ser encaminhados sem reprocessá-la.
    pub fn poll_frame(&mut self, input: &mut Input) -> (bool, Vec<Event>) {
        let events = self.poll_raw_events();
        input.begin_frame();
        input.update(&events);
        let keep_running = !events.iter().any(Self::requests_exit);
        (keep_running, events)
    }

    fn requests_exit(event: &Event) -> bool {
        matches!(
            event,
            Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
        )
    }
}

/// Inicializa SDL2 e o subsistema de vídeo, usando KMSDRM apenas sem escolha explícita.
pub fn init_sdl() -> Result<Sdl, PlatformError> {
    if std::env::var_os("SDL_VIDEODRIVER").is_none() {
        hint::set("SDL_VIDEODRIVER", "KMSDRM");
    }

    // SDL_KMSDRM_REQUIRE_DRM_MASTER controla a exigência de DRM master.
    // SDL_KMSDRM_DEVICE_INDEX seleciona a GPU quando há mais de um dispositivo DRM.
    // SDL_VIDEO_EGL_DRIVER e SDL_VIDEO_GL_DRIVER permitem indicar drivers EGL e OpenGL.
    let ctx = sdl2::init().map_err(PlatformError::Initialization)?;
    let video = ctx.video().map_err(PlatformError::VideoSubsystem)?;
    let event_pump = ctx.event_pump().map_err(PlatformError::EventPump)?;
    log::info!(
        "driver de vídeo SDL2 ativo: {}",
        video.current_video_driver()
    );

    Ok(Sdl {
        event_pump,
        video,
        ctx,
    })
}
