//! Controla um retângulo com as setas usando o loop de timestep fixo da engine.
//! Status: exemplo funcional de input direcional e movimento.

use anyhow::Result;
use giochi_engine::{
    core::{
        engine::{EngineConfig, GameEngine},
        game_loop::Game,
    },
    platform::{sdl::init_sdl, window::RenderContext},
};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

struct MovingSprite {
    x: f32,
    y: f32,
    direction: (f32, f32),
}

impl Game for MovingSprite {
    fn update(&mut self, dt: f32) {
        self.x = (self.x + self.direction.0 * 240.0 * dt).clamp(0.0, 600.0);
        self.y = (self.y + self.direction.1 * 240.0 * dt).clamp(0.0, 440.0);
    }

    fn render(&mut self, context: &mut RenderContext<'_>) {
        context.clear(Color::RGB(18, 22, 32));
        context.fill_rect(
            self.x as i32,
            self.y as i32,
            40,
            40,
            Color::RGB(80, 190, 245),
        );
        context.present();
    }
}

fn main() -> Result<()> {
    let mut sdl = init_sdl()?;
    let video = sdl.video();
    let config = EngineConfig {
        width: 640,
        height: 480,
    };
    let window = video
        .window("GIOCHI-ENGINE — Sprite móvel", config.width, config.height)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas().accelerated().build()?;
    let mut context = RenderContext::new(&mut canvas);
    let mut engine = GameEngine::new(config);
    let mut game = MovingSprite {
        x: 300.0,
        y: 220.0,
        direction: (0.0, 0.0),
    };

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
    });
    Ok(())
}

fn set_direction(game: &mut MovingSprite, key: Keycode, pressed: bool) {
    match key {
        Keycode::Left => game.direction.0 = if pressed { -1.0 } else { 0.0 },
        Keycode::Right => game.direction.0 = if pressed { 1.0 } else { 0.0 },
        Keycode::Up => game.direction.1 = if pressed { -1.0 } else { 0.0 },
        Keycode::Down => game.direction.1 = if pressed { 1.0 } else { 0.0 },
        _ => {}
    }
}
