//! Retângulo axis-aligned de coordenadas inteiras.
//! Status: interseção axis-aligned implementada.

/// Retângulo para limites e colisões simples.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// Coordenada horizontal da origem.
    pub x: i32,
    /// Coordenada vertical da origem.
    pub y: i32,
    /// Largura não negativa.
    pub width: u32,
    /// Altura não negativa.
    pub height: u32,
}

impl Rect {
    /// Cria um retângulo com origem e dimensões.
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Informa se o retângulo não tem área.
    pub fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Verifica interseção com outro retângulo.
    ///
    /// Retângulos sem área (largura ou altura zero) nunca interseptam: encostar
    /// nas bordas não é sobreposição. A comparação usa `i64` para não estourar
    /// perto dos limites de `i32`.
    pub fn intersects(self, other: Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return false;
        }
        let right = i64::from(self.x) + i64::from(self.width);
        let bottom = i64::from(self.y) + i64::from(self.height);
        let other_right = i64::from(other.x) + i64::from(other.width);
        let other_bottom = i64::from(other.y) + i64::from(other.height);
        i64::from(self.x) < other_right
            && right > i64::from(other.x)
            && i64::from(self.y) < other_bottom
            && bottom > i64::from(other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_overlapping_rectangles() {
        let a = Rect::new(0, 0, 10, 10);
        assert!(a.intersects(Rect::new(5, 5, 10, 10)));
        assert!(a.intersects(Rect::new(0, 0, 10, 10)), "igual a si mesmo");
        assert!(a.intersects(Rect::new(2, 2, 3, 3)), "contido");
        assert!(a.intersects(Rect::new(-5, -5, 20, 20)), "contendo");
    }

    #[test]
    fn rejects_rectangles_separated_on_any_axis() {
        let a = Rect::new(0, 0, 10, 10);
        assert!(!a.intersects(Rect::new(10, 0, 5, 5)), "encostado à direita");
        assert!(
            !a.intersects(Rect::new(-5, 0, 5, 5)),
            "encostado à esquerda"
        );
        assert!(!a.intersects(Rect::new(0, 10, 5, 5)), "abaixo");
        assert!(!a.intersects(Rect::new(0, -5, 5, 5)), "acima");
        assert!(!a.intersects(Rect::new(50, 50, 1, 1)), "distante");
    }

    #[test]
    fn handles_empty_and_negative_coordinates() {
        let empty = Rect::new(5, 5, 0, 0);
        assert!(
            !empty.intersects(Rect::new(0, 0, 10, 10)),
            "sem área não colide"
        );
        assert!(Rect::new(-10, -10, 5, 5).intersects(Rect::new(-8, -8, 1, 1)));
    }

    #[test]
    fn does_not_overflow_near_i32_limits() {
        let huge = Rect::new(i32::MAX - 2, i32::MAX - 2, u32::MAX, u32::MAX);
        let other = Rect::new(i32::MIN, i32::MIN, 10, 10);
        // A conta em i64 evita overflow e conclui corretamente que não há interseção.
        assert!(!huge.intersects(other));
    }
}
