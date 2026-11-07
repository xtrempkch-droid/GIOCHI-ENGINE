//! Testes de integração headless com SDL2 dummy e operações matemáticas básicas.
//! Status: cache, interseção, encerramento do loop e renderização headless cobertos.

use giochi_engine::{
    assets::manager::AssetManager,
    core::game_loop::{self, Game},
    math::rect::Rect,
    platform::{init_sdl, Window},
    render::{renderer::RenderContext, texture::Texture, RenderError},
};
use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
};

#[test]
fn asset_manager_reuses_inserted_values() {
    let mut manager = AssetManager::default();
    manager.insert("player", 7_u32);
    assert_eq!(manager.get("player"), Some(&7));
}

#[test]
fn rectangles_detect_intersection() {
    let first = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let second = Rect {
        x: 9,
        y: 9,
        width: 3,
        height: 3,
    };
    assert!(first.intersects(second));
}

#[test]
fn game_loop_exits_when_escape_is_queued() {
    std::env::set_var("SDL_VIDEODRIVER", "dummy");
    let mut sdl = init_sdl().expect("o driver SDL2 dummy deve inicializar");
    let mut window =
        Window::new(&sdl, "teste headless", 32, 32).expect("o driver dummy deve criar uma janela");
    sdl.ctx
        .event()
        .expect("o subsistema de eventos deve estar ativo")
        .push_event(Event::KeyDown {
            timestamp: 0,
            window_id: 0,
            keycode: Some(Keycode::Escape),
            scancode: None,
            keymod: Mod::NOMOD,
            repeat: false,
        })
        .expect("o evento Escape deve ser enfileirado");
    let mut game = CountingGame::default();

    game_loop::run(&mut game, &mut window, &mut sdl).expect("o loop deve encerrar sem erro");

    assert_eq!(game.updates, 0);
    assert_eq!(game.renders, 0);

    let (mut context, loader) = window
        .render_context_with_loader()
        .expect("SDL_image deve inicializar com o driver dummy");
    let texture = Texture::from_rgba(
        &loader,
        2,
        2,
        &[
            255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
        ],
    )
    .expect("a textura RGBA deve ser criada");
    assert_eq!(texture.dimensions(), (2, 2));
    let black = giochi_engine::math::color::Color::rgb(0, 0, 0);
    let red = giochi_engine::math::color::Color::rgb(255, 0, 0);
    context.clear(black);
    context
        .draw_rect(Rect::new(1, 1, 4, 4), red)
        .expect("retângulo deve ser desenhado");
    context
        .draw_texture(&texture, Rect::new(8, 8, 16, 16))
        .expect("textura deve ser desenhada");
    context.present();

    assert!(matches!(
        Texture::from_rgba(&loader, 2, 2, &[0; 4]),
        Err(RenderError::InvalidImageData { .. })
    ));
}

#[derive(Default)]
struct CountingGame {
    updates: usize,
    renders: usize,
}

impl Game for CountingGame {
    fn update(&mut self, _dt: f32) {
        self.updates += 1;
    }

    fn render(&mut self, _context: &mut RenderContext<'_>) -> Result<(), RenderError> {
        self.renders += 1;
        Ok(())
    }
}
