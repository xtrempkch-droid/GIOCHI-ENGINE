//! Retângulo axis-aligned de coordenadas inteiras.
//! Status: interseção axis-aligned implementada.

/// Retângulo para limites e colisões simples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Verifica interseção com outro retângulo.
    pub fn intersects(self, other: Self) -> bool {
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
