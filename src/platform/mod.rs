//! Integração com SDL2 e recursos específicos do sistema operacional.
//! Status: inicialização SDL2 e adaptador de janela implementados.

use thiserror::Error;

pub mod sdl;
pub mod window;

pub use sdl::{init_sdl, Sdl, SdlEvent};
pub use window::{RenderContext, Window};

/// Falhas na inicialização SDL2, criação de janela ou recursos de plataforma.
#[derive(Debug, Error)]
pub enum PlatformError {
    /// O SDL2 não pôde ser inicializado.
    #[error(
        "não foi possível inicializar o SDL2: {0}; verifique permissões de /dev/dri/card0 e grupos input/video"
    )]
    Initialization(String),
    /// O subsistema de vídeo não pôde ser aberto.
    #[error("não foi possível inicializar o vídeo SDL2: {0}; verifique o driver SDL_VIDEODRIVER")]
    VideoSubsystem(String),
    /// A fila de eventos não pôde ser criada.
    #[error("não foi possível criar a fila de eventos SDL2: {0}")]
    EventPump(String),
    /// A janela SDL2 não pôde ser criada.
    #[error("não foi possível criar a janela SDL2: {0}")]
    Window(String),
    /// O canvas de desenho não pôde ser criado.
    #[error("não foi possível criar o canvas SDL2: {0}")]
    Canvas(String),
}
