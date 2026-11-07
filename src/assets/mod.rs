//! Carregamento e cache de recursos do jogo.
//! Status: leitura, texturas, áudio, fontes e cache com estatísticas implementados.
//!
//! `loader` concentra o acesso ao disco e aos backends SDL (imagem, mixer e TTF);
//! `manager` mantém os recursos já carregados compartilhados por `Rc`.

pub mod loader;
pub mod manager;

use thiserror::Error;

pub use loader::{
    load_music, load_sound, load_texture, load_texture_bytes, read_bytes, validate_asset_path,
    FontLoader,
};
pub use manager::{AssetManager, CacheStats, FontCache, ResourceManager};

/// Falhas ao localizar, ler ou decodificar um recurso.
#[derive(Debug, Error)]
pub enum AssetError {
    /// O caminho não existe ou não é um arquivo regular.
    #[error("recurso não encontrado ou inválido: {path}")]
    InvalidPath {
        /// Caminho informado.
        path: String,
        /// Motivo devolvido pelo sistema de arquivos.
        message: String,
    },
    /// A leitura do arquivo falhou.
    #[error("falha ao ler '{path}': {message}")]
    Io {
        /// Caminho informado.
        path: String,
        /// Motivo devolvido pelo sistema de arquivos.
        message: String,
    },
    /// A decodificação da imagem falhou.
    #[error("falha ao carregar a textura '{path}': {message}")]
    Texture {
        /// Caminho informado.
        path: String,
        /// Motivo devolvido pelo SDL_image.
        message: String,
    },
    /// O carregamento de áudio falhou.
    #[error("falha ao carregar o áudio '{path}': {message}")]
    Audio {
        /// Caminho informado.
        path: String,
        /// Motivo devolvido pelo SDL_mixer.
        message: String,
    },
    /// A fonte não pôde ser aberta.
    #[error("falha ao carregar a fonte '{path}' em {size}pt: {message}")]
    Font {
        /// Caminho informado.
        path: String,
        /// Corpo solicitado.
        size: u16,
        /// Motivo devolvido pelo SDL_ttf.
        message: String,
    },
}
