//! Estado de renderização lógico compartilhado pela camada SDL2.
//! Status: configuração lógica básica; renderer SDL2 é fornecido pela camada platform.

/// Configuração simples da área de desenho.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RendererConfig {
    /// Largura da área lógica.
    pub width: u32,
    /// Altura da área lógica.
    pub height: u32,
}
