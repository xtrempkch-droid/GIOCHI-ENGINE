//! Tipos de renderização 2D e primitivas reutilizáveis.
//! Status: desenho de primitivas, texturas, sprites e transformação de câmera implementados.

pub mod camera;
pub mod renderer;
pub mod sprite;
pub mod texture;

/// Erros de carregamento de imagens e comandos de renderização.
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    /// O SDL2 recusou um comando de desenho.
    #[error("falha ao desenhar: {0}")]
    Draw(String),
    /// O SDL_image não conseguiu carregar ou inicializar uma imagem.
    #[error("falha do SDL_image: {0}")]
    Image(String),
    /// O SDL2 não conseguiu criar ou atualizar uma textura.
    #[error("falha ao criar textura: {0}")]
    Texture(String),
    /// As dimensões excedem o limite representável.
    #[error("dimensões da imagem inválidas ou muito grandes")]
    InvalidImageDimensions,
    /// O número de bytes não corresponde às dimensões RGBA solicitadas.
    #[error("dados RGBA inválidos: esperado {expected} bytes, recebido {actual}")]
    InvalidImageData { expected: usize, actual: usize },
}
