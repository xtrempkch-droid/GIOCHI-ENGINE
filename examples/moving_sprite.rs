//! Move uma textura procedural com as setas; Escape ou Quit encerra o programa.
//! Status: exemplo funcional de textura, sprite, primitivas e eventos de teclado.

use std::rc::Rc;

use anyhow::Result;
use giochi_engine::{
    core::engine::{EngineConfig, GameEngine},
    core::game_loop::Game,
    math::{color::Color, rect::Rect},
    platform::{init_sdl, Window},
    render::{
        renderer::RenderContext,
        sprite::Sprite,
        texture::{Texture, TextureLoader},
        RenderError,
    },
};
use sdl2::{event::Event, keyboard::Keycode};

struct MovingSprite<'a> {
    sprite: Sprite<'a>,
    x: f32,
    y: f32,
    direction: (f32, f32),
}

impl Game for MovingSprite<'_> {
    fn update(&mut self, dt: f32) {
        self.x = (self.x + self.direction.0 * 240.0 * dt).clamp(0.0, 600.0);
        self.y = (self.y + self.direction.1 * 240.0 * dt).clamp(0.0, 440.0);
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
        direction: (0.0, 0.0),
    };

    let mut engine = GameEngine::new(config);
    engine.run(&mut game, &mut context, |game| {
        let mut running = true;
        for event in sdl.poll_raw_events() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => running = false,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => set_direction(game, key, true),
                Event::KeyUp {
                    keycode: Some(key), ..
                } => set_direction(game, key, false),
                _ => {}
            }
        }
        running
    })?;
    Ok(())
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

fn set_direction(game: &mut MovingSprite<'_>, key: Keycode, pressed: bool) {
    match key {
        Keycode::Left => game.direction.0 = if pressed { -1.0 } else { 0.0 },
        Keycode::Right => game.direction.0 = if pressed { 1.0 } else { 0.0 },
        Keycode::Up => game.direction.1 = if pressed { -1.0 } else { 0.0 },
        Keycode::Down => game.direction.1 = if pressed { 1.0 } else { 0.0 },
        _ => {}
    }
}
