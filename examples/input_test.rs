//! Demonstra a entrada unificada: movimento, cor e ponteiro.
//! Status: exemplo funcional de teclado, mouse e gamepad.
//!
//! Controles: setas/WASD movem, Espaço troca a cor e o botão direito do mouse
//! reposiciona o quadrado sob o ponteiro. O analógico esquerdo também move.

use anyhow::Result;
use giochi_engine::{
    core::game_loop::{self, Game},
    input::{GamepadButton, Input, MouseButton},
    math::{color::Color, rect::Rect},
    platform::{init_sdl, Window},
    render::{renderer::RenderContext, RenderError},
};
use glam::Vec2;
use sdl2::keyboard::Keycode;

const SPEED: f32 = 260.0;
const SIZE: f32 = 48.0;

const PALETTE: [Color; 5] = [
    Color::rgb(245, 120, 70),
    Color::rgb(90, 210, 140),
    Color::rgb(80, 160, 245),
    Color::rgb(215, 105, 220),
    Color::rgb(245, 220, 90),
];

struct InputDemo {
    position: Vec2,
    color_index: usize,
}

impl Game for InputDemo {
    fn update(&mut self, dt: f32, input: &Input) {
        let direction =
            (input.keyboard.movement() + input.gamepad.left_stick()).clamp_length_max(1.0);

        if input.mouse.is_down(MouseButton::Right) {
            let (x, y) = input.mouse.position();
            self.position = Vec2::new(x as f32, y as f32);
        } else {
            self.position += direction * SPEED * dt;
        }
        self.position = self.position.clamp(
            Vec2::splat(SIZE / 2.0),
            Vec2::new(640.0, 480.0) - SIZE / 2.0,
        );

        let color_requested = input.keyboard.is_pressed_this_frame(Keycode::Space)
            || input.gamepad.is_pressed_this_frame(GamepadButton::South);
        if color_requested {
            self.color_index = (self.color_index + 1) % PALETTE.len();
        }
    }

    fn render(&mut self, context: &mut RenderContext<'_>) -> Result<(), RenderError> {
        context.clear(Color::rgb(16, 18, 26));
        let half = SIZE / 2.0;
        context.draw_rect(
            Rect::new(
                (self.position.x - half).round() as i32,
                (self.position.y - half).round() as i32,
                SIZE as u32,
                SIZE as u32,
            ),
            PALETTE[self.color_index],
        )
    }
}

fn main() -> Result<()> {
    let mut sdl = init_sdl()?;
    let mut window = Window::new(&sdl, "GIOCHI-ENGINE — Entrada", 640, 480)?;
    let mut game = InputDemo {
        position: Vec2::new(320.0, 240.0),
        color_index: 0,
    };
    game_loop::run(&mut game, &mut window, &mut sdl)
}
