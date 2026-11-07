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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_is_opaque() {
        let color = Color::rgb(10, 20, 30);
        assert_eq!((color.red, color.green, color.blue), (10, 20, 30));
        assert_eq!(color.alpha, u8::MAX);
    }

    #[test]
    fn rgba_keeps_alpha() {
        let color = Color::rgba(1, 2, 3, 4);
        assert_eq!(
            (color.red, color.green, color.blue, color.alpha),
            (1, 2, 3, 4)
        );
    }

    #[test]
    fn default_is_transparent_black() {
        let color = Color::default();
        assert_eq!(
            (color.red, color.green, color.blue, color.alpha),
            (0, 0, 0, 0)
        );
    }
}
