//! Desenha um retângulo colorido em SDL2 e encerra com Escape ou evento de saída.
//! Status: exemplo funcional de janela e desenho SDL2.

use std::time::Duration;

use anyhow::Result;
use giochi_engine::platform::sdl::init_sdl;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

fn main() -> Result<()> {
    let mut sdl = init_sdl()?;
    let video = sdl.video();
    let window = video
        .window("GIOCHI-ENGINE — Retângulo", 640, 480)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas().accelerated().build()?;
    let mut running = true;

    while running {
        for event in sdl.poll_raw_events() {
            if matches!(
                event,
                Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    }
            ) {
                running = false;
            }
        }
        canvas.set_draw_color(Color::RGB(20, 24, 35));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(245, 120, 70));
        canvas
            .fill_rect(sdl2::rect::Rect::new(220, 160, 200, 160))
            .map_err(anyhow::Error::msg)?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
