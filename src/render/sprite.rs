//! Transformação básica de um sprite 2D.
//! Status: posição e escala implementadas; integração com textura/renderizador é parcial.

use glam::Vec2;

/// Posição e escala de um sprite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite {
    /// Posição no mundo.
    pub position: Vec2,
    /// Escala horizontal e vertical.
    pub scale: Vec2,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            scale: Vec2::ONE,
        }
    }
}
