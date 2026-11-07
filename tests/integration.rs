//! Testes de integração que não exigem janela nem dispositivo de áudio.
//! Status: testes headless de cache de assets e colisão retangular.

use giochi_engine::{
    assets::manager::AssetManager,
    core::game_loop::{self, Game},
    math::rect::Rect,
    platform::{init_sdl, window::RenderContext, Window},
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

    fn render(&mut self, _context: &mut RenderContext<'_>) {
        self.renders += 1;
    }
}
