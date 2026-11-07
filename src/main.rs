//! Demo mínimo que inicializa SDL2, limpa a tela e encerra com Escape.

use anyhow::Result;
use giochi_engine::{
    core::game_loop::{self, Game},
    math::color::Color,
    platform::{init_sdl, Window},
    render::{renderer::RenderContext, RenderError},
};

struct DemoGame;

impl Game for DemoGame {
    fn update(&mut self, _dt: f32) {}

    fn render(&mut self, context: &mut RenderContext<'_>) -> Result<(), RenderError> {
        context.clear(Color::rgb(0, 0, 0));
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let mut sdl = init_sdl()?;
    let mut window = Window::new(&sdl, "GIOCHI-SYSTEM Engine", 1280, 720)?;
    let mut game = DemoGame;
    game_loop::run(&mut game, &mut window, &mut sdl)
}
