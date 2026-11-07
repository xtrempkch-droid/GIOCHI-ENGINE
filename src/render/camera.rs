//! Câmera 2D descrita por posição e zoom.
//! Status: dados de câmera implementados; aplicação da transformação ainda não implementada.

use glam::Vec2;

/// Transformação da visão do mundo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    /// Centro da câmera em coordenadas de mundo.
    pub position: Vec2,
    /// Fator de zoom; 1 representa escala natural.
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}
