//! Desenha um retângulo vermelho que percorre uma órbita sobre fundo azul.
//! Status: exemplo funcional de primitivas e loop com timestep fixo.

use anyhow::Result;
use giochi_engine::{
    core::game_loop::{self, Game},
    math::{color::Color, rect::Rect},
    platform::{init_sdl, Window},
    render::{renderer::RenderContext, RenderError},
};
use glam::Vec2;

struct OrbitingRect {
    elapsed: f32,
}

impl Game for OrbitingRect {
    fn update(&mut self, dt: f32) {
        self.elapsed += dt;
    }

    fn render(&mut self, context: &mut RenderContext<'_>) -> Result<(), RenderError> {
        context.clear(Color::rgb(28, 72, 150));
        let center = Vec2::new(320.0, 240.0);
        let position = center + Vec2::new(self.elapsed.cos(), self.elapsed.sin()) * 120.0;
        context.draw_rect(
            Rect::new(
                (position.x - 24.0).round() as i32,
                (position.y - 24.0).round() as i32,
                48,
                48,
            ),
            Color::rgb(235, 70, 65),
        )
    }
}

fn main() -> Result<()> {
    let mut sdl = init_sdl()?;
    let mut window = Window::new(&sdl, "GIOCHI-ENGINE — Retângulo", 640, 480)?;
    game_loop::run(&mut OrbitingRect { elapsed: 0.0 }, &mut window, &mut sdl)
}
