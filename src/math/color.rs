//! Cor RGBA de oito bits por canal.
//! Status: tipo RGBA e construtor RGB implementados.

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
}
