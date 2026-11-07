//! Testes de integração headless com SDL2 dummy e operações matemáticas básicas.
//! Status: cache, interseção, encerramento do loop e renderização headless cobertos.

use giochi_engine::{
    assets::manager::AssetManager,
    core::game_loop::{self, Game},
    input::Input,
    math::rect::Rect,
    platform::{init_sdl, Window},
    render::{renderer::RenderContext, texture::Texture, RenderError},
};
use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
};
use std::sync::{Mutex, MutexGuard};

/// O SDL2 permite apenas um contexto por processo; os testes que o inicializam
/// são serializados por este cadeado.
static SDL_LOCK: Mutex<()> = Mutex::new(());

fn lock_sdl() -> MutexGuard<'static, ()> {
    SDL_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

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
    let _guard = lock_sdl();
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

#[test]
fn draw_rect_writes_expected_pixels_in_headless_mode() {
    let _guard = lock_sdl();
    std::env::set_var("SDL_VIDEODRIVER", "dummy");
    let sdl = init_sdl().expect("o driver SDL2 dummy deve inicializar");
    let mut window =
        Window::new(&sdl, "teste de pixels", 32, 32).expect("o driver dummy deve criar uma janela");
    let (mut context, _loader) = window
        .render_context_with_loader()
        .expect("o contexto e o loader devem ser criados");

    let background = giochi_engine::math::color::Color::rgb(0, 0, 0);
    let foreground = giochi_engine::math::color::Color::rgb(255, 0, 0);
    context.clear(background);
    context
        .draw_rect(Rect::new(8, 8, 16, 16), foreground)
        .expect("retângulo deve ser desenhado");

    let inside = context
        .read_pixels(Rect::new(16, 16, 1, 1))
        .expect("pixel interno deve ser lido");
    assert_eq!(
        &inside[..3],
        &[255, 0, 0],
        "interior do retângulo deve ser vermelho"
    );

    let outside = context
        .read_pixels(Rect::new(1, 1, 1, 1))
        .expect("pixel externo deve ser lido");
    assert_eq!(&outside[..3], &[0, 0, 0], "fundo deve permanecer preto");
}

#[derive(Default)]
struct CountingGame {
    updates: usize,
    renders: usize,
}

impl Game for CountingGame {
    fn update(&mut self, _dt: f32, _input: &Input) {
        self.updates += 1;
    }

    fn render(&mut self, _context: &mut RenderContext<'_>) -> Result<(), RenderError> {
        self.renders += 1;
        Ok(())
    }
}

/// PNG 2x2 totalmente vermelho, usado para cobrir a decodificação via SDL_image.
const RED_PNG: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 2, 0, 0, 0, 2, 8, 6, 0,
    0, 0, 114, 182, 13, 36, 0, 0, 0, 17, 73, 68, 65, 84, 120, 218, 99, 248, 207, 192, 240, 31, 132,
    25, 96, 12, 0, 71, 202, 7, 249, 26, 182, 241, 169, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96,
    130,
];

#[test]
fn texture_is_decoded_from_png_bytes() {
    let _guard = lock_sdl();
    std::env::set_var("SDL_VIDEODRIVER", "dummy");
    let sdl = init_sdl().expect("o driver SDL2 dummy deve inicializar");
    let mut window =
        Window::new(&sdl, "teste de png", 32, 32).expect("o driver dummy deve criar uma janela");
    let (_context, loader) = window
        .render_context_with_loader()
        .expect("o contexto e o loader devem ser criados");

    let texture = Texture::from_bytes(&loader, RED_PNG).expect("PNG deve ser decodificado");
    assert_eq!(texture.dimensions(), (2, 2));
}
