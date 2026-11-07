//! Inicialização SDL2 com seleção condicional do driver de vídeo KMSDRM.
//! Status: inicialização e diagnóstico tipado implementados.

use sdl2::{hint, Sdl};
use thiserror::Error;

/// Falha ao inicializar o SDL2.
#[derive(Debug, Error)]
pub enum PlatformError {
    /// O SDL2 recusou a inicialização.
    #[error(
        "não foi possível inicializar o SDL2: {0}. Verifique as permissões de /dev/dri e os grupos input e video"
    )]
    Initialization(String),
}

/// Inicializa SDL2 usando KMSDRM por padrão em dispositivos sem driver configurado.
pub fn init_sdl() -> Result<Sdl, PlatformError> {
    if std::env::var_os("SDL_VIDEODRIVER").is_none() {
        hint::set("SDL_VIDEODRIVER", "KMSDRM");
    }

    // SDL_KMSDRM_REQUIRE_DRM_MASTER controla a exigência de DRM master.
    // SDL_KMSDRM_DEVICE_INDEX seleciona a GPU quando há mais de um dispositivo DRM.
    // SDL_VIDEO_EGL_DRIVER e SDL_VIDEO_GL_DRIVER permitem indicar drivers EGL e OpenGL.
    sdl2::init().map_err(PlatformError::Initialization)
}
