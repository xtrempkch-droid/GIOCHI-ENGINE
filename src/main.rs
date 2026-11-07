//! Demo mínimo da fachada `Engine`: janela preta e saída por Escape.

use anyhow::Result;
use giochi_engine::{
    core::engine::{bootstrap, EngineConfig},
    input::Input,
    math::color::Color,
    render::{renderer::RenderContext, RenderError},
    scene::Scene,
};

/// Cena inicial do binário de demonstração.
struct DemoScene;

impl Scene for DemoScene {
    fn update(&mut self, _dt: f32, _input: &Input) {}

    fn render(&mut self, context: &mut RenderContext<'_>) -> Result<(), RenderError> {
        context.clear(Color::rgb(0, 0, 0));
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let config = EngineConfig::new("GIOCHI-SYSTEM Engine", 1280, 720);
    bootstrap(config, Box::new(DemoScene)).map_err(anyhow::Error::msg)
}
