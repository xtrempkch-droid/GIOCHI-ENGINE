//! Move uma textura procedural com setas, WASD ou analógico do gamepad.
//! Status: exemplo funcional de textura, sprite e entrada unificada.

use std::rc::Rc;

use anyhow::Result;
use giochi_engine::{
    core::engine::{EngineConfig, GameEngine},
    core::game_loop::Game,
    input::Input,
    math::{color::Color, rect::Rect},
    platform::{init_sdl, Window},
    render::{
        renderer::RenderContext,
        sprite::Sprite,
        texture::{Texture, TextureLoader},
        RenderError,
    },
};

struct MovingSprite<'a> {
    sprite: Sprite<'a>,
    x: f32,
    y: f32,
}

impl Game for MovingSprite<'_> {
    fn update(&mut self, dt: f32, input: &Input) {
        let direction =
            (input.keyboard.movement() + input.gamepad.left_stick()).clamp_length_max(1.0);
        self.x = (self.x + direction.x * 240.0 * dt).clamp(0.0, 600.0);
        self.y = (self.y + direction.y * 240.0 * dt).clamp(0.0, 440.0);
    }

    fn render(&mut self, context: &mut RenderContext<'_>) -> Result<(), RenderError> {
        context.clear(Color::rgb(18, 22, 32));
        self.sprite.draw(
            context,
            Rect::new(self.x.round() as i32, self.y.round() as i32, 40, 40),
        )
    }
}

fn main() -> Result<()> {
    let mut sdl = init_sdl()?;
    let config = EngineConfig {
        width: 640,
        height: 480,
    };
    let mut window = Window::new(
        &sdl,
        "GIOCHI-ENGINE — Sprite móvel",
        config.width,
        config.height,
    )?;
    let (mut context, loader) = window.render_context_with_loader()?;
    let texture = Rc::new(create_procedural_texture(&loader)?);
    let mut game = MovingSprite {
        sprite: Sprite::new(texture),
        x: 300.0,
        y: 220.0,
    };

    let mut engine = GameEngine::new(config);
    engine.run(&mut game, &mut context, &mut sdl)
}

fn create_procedural_texture<'loader, 'window>(
    loader: &'loader TextureLoader<'window>,
) -> Result<Texture<'loader>, RenderError> {
    let mut pixels = Vec::with_capacity(64 * 64 * 4);
    for y in 0..64 {
        for x in 0..64 {
            let border = x < 4 || y < 4 || x >= 60 || y >= 60;
            let checker = (x / 8 + y / 8) % 2 == 0;
            let color = if border {
                [245, 220, 90, 255]
            } else if checker {
                [50, 180, 230, 255]
            } else {
                [35, 115, 200, 255]
            };
            pixels.extend_from_slice(&color);
        }
    }
    Texture::from_rgba(loader, 64, 64, &pixels)
}
