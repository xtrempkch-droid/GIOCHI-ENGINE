//! Câmera 2D que transforma coordenadas do mundo em coordenadas da viewport.
//! Status: conversão de pontos e retângulos mundo/tela implementada.

use crate::math::{rect::Rect, vec2::Vec2};

/// Transformação da visão do mundo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    /// Centro da câmera em coordenadas de mundo.
    pub position: Vec2,
    /// Fator de zoom; 1 representa escala natural.
    pub zoom: f32,
    /// Área em pixels na qual o mundo será desenhado.
    pub viewport: Rect,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            viewport: Rect::default(),
        }
    }
}

impl Camera {
    /// Converte um ponto do mundo para a posição correspondente na tela.
    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        let viewport_center = Vec2::new(
            self.viewport.x as f32 + self.viewport.width as f32 / 2.0,
            self.viewport.y as f32 + self.viewport.height as f32 / 2.0,
        );
        viewport_center + (world - self.position) * self.zoom
    }

    /// Transforma origem e dimensões de um retângulo do mundo para a tela.
    pub fn apply(&self, rect_world: Rect) -> Rect {
        let screen_position =
            self.world_to_screen(Vec2::new(rect_world.x as f32, rect_world.y as f32));
        Rect::new(
            screen_position.x.round() as i32,
            screen_position.y.round() as i32,
            (rect_world.width as f32 * self.zoom).abs().round() as u32,
            (rect_world.height as f32 * self.zoom).abs().round() as u32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_transforms_world_points_to_viewport_coordinates() {
        let camera = Camera {
            position: Vec2::new(10.0, 20.0),
            zoom: 2.0,
            viewport: Rect::new(100, 50, 400, 200),
        };

        assert_eq!(
            camera.world_to_screen(Vec2::new(15.0, 25.0)),
            Vec2::new(310.0, 160.0)
        );
    }

    #[test]
    fn camera_transforms_world_rectangles_including_zoom() {
        let camera = Camera {
            position: Vec2::new(10.0, 20.0),
            zoom: 2.0,
            viewport: Rect::new(100, 50, 400, 200),
        };

        assert_eq!(
            camera.apply(Rect::new(10, 20, 20, 10)),
            Rect::new(300, 150, 40, 20)
        );
    }
}
