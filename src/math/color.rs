//! Cor RGBA de oito bits por canal.
//! Status: tipo de cor e construtores RGB/RGBA implementados.

/// Cor com canais vermelho, verde, azul e alfa.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Canal vermelho.
    pub red: u8,
    /// Canal verde.
    pub green: u8,
    /// Canal azul.
    pub blue: u8,
    /// Opacidade.
    pub alpha: u8,
}

impl Color {
    /// Cria uma cor opaca RGB.
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: u8::MAX,
        }
    }

    /// Cria uma cor RGBA.
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}
