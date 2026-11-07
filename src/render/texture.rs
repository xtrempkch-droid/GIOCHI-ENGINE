//! Metadados de textura sem propriedade de recursos SDL2.
//! Status: metadados implementados; carregamento e desenho de textura ainda não implementados.

/// Dimensões conhecidas de um recurso de imagem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Texture {
    /// Largura em pixels.
    pub width: u32,
    /// Altura em pixels.
    pub height: u32,
}
