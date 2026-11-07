//! Testes de integração headless com SDL2 dummy e operações matemáticas básicas.
//! Status: cache, interseção, encerramento do loop e renderização headless cobertos.

use giochi_engine::{
    assets::AssetError,
    assets::{manager::AssetManager, FontLoader, ResourceManager},
    audio::{AudioSystem, MixerConfig},
    core::engine::{Engine, EngineConfig},
    core::game_loop::{self, Game},
    input::Input,
    math::{color::Color, rect::Rect},
    platform::{init_sdl, Window},
    render::{renderer::RenderContext, texture::Texture, RenderError},
    scene::{Scene, SceneContext},
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

fn write_test_wave(path: &std::path::Path) {
    use std::io::Write;

    let sample_rate = 8_000_u32;
    let samples: Vec<i16> = (0..800)
        .map(|index| {
            let phase = std::f32::consts::TAU * 440.0 * index as f32 / sample_rate as f32;
            (phase.sin() * 8_000.0) as i16
        })
        .collect();
    let data_size = (samples.len() * 2) as u32;

    let mut file = std::fs::File::create(path).expect("o WAV de teste deve ser criado");
    file.write_all(b"RIFF").unwrap();
    file.write_all(&(36 + data_size).to_le_bytes()).unwrap();
    file.write_all(b"WAVEfmt ").unwrap();
    file.write_all(&16_u32.to_le_bytes()).unwrap();
    file.write_all(&1_u16.to_le_bytes()).unwrap();
    file.write_all(&1_u16.to_le_bytes()).unwrap();
    file.write_all(&sample_rate.to_le_bytes()).unwrap();
    file.write_all(&(sample_rate * 2).to_le_bytes()).unwrap();
    file.write_all(&2_u16.to_le_bytes()).unwrap();
    file.write_all(&16_u16.to_le_bytes()).unwrap();
    file.write_all(b"data").unwrap();
    file.write_all(&data_size.to_le_bytes()).unwrap();
    for sample in samples {
        file.write_all(&sample.to_le_bytes()).unwrap();
    }
}

#[test]
fn resource_manager_loads_texture_once_for_repeated_requests() {
    let _guard = lock_sdl();
    std::env::set_var("SDL_VIDEODRIVER", "dummy");
    let sdl = init_sdl().expect("o driver SDL2 dummy deve inicializar");
    let mut window = Window::new(&sdl, "cache de texturas", 32, 32)
        .expect("o driver dummy deve criar uma janela");
    let (_context, loader) = window
        .render_context_with_loader()
        .expect("o contexto e o loader devem ser criados");

    let directory = tempfile::tempdir().expect("o diretório temporário deve ser criado");
    let path = directory.path().join("red.png");
    std::fs::write(&path, RED_PNG).expect("o PNG de teste deve ser gravado");

    let mut resources = ResourceManager::new();
    let first = resources
        .texture(&loader, &path)
        .expect("a primeira requisição deve carregar o PNG");
    let second = resources
        .texture(&loader, &path)
        .expect("a segunda requisição deve vir do cache");

    assert!(std::rc::Rc::ptr_eq(&first, &second));
    assert_eq!(resources.texture_count(), 1);
    assert_eq!(
        resources.stats().loads(),
        1,
        "o arquivo deve ser carregado uma vez"
    );
    assert_eq!(resources.stats().hits(), 1);
    assert_eq!(resources.stats().requests(), 2);
}

#[test]
fn resource_manager_loads_sound_once_for_repeated_requests() {
    let _guard = lock_sdl();
    std::env::set_var("SDL_AUDIODRIVER", "dummy");
    let audio = AudioSystem::init(MixerConfig::MONO).expect("o driver de áudio dummy deve abrir");

    let directory = tempfile::tempdir().expect("o diretório temporário deve ser criado");
    let path = directory.path().join("tone.wav");
    write_test_wave(&path);

    let mut resources = ResourceManager::new();
    let first = resources.sound(&path).expect("o WAV deve ser carregado");
    let second = resources
        .sound(&path)
        .expect("a segunda requisição deve vir do cache");

    assert!(std::rc::Rc::ptr_eq(&first, &second));
    assert_eq!(resources.sound_count(), 1);
    assert_eq!(resources.stats().loads(), 1);
    assert_eq!(resources.stats().hits(), 1);
    assert!(first.volume() > 0.0);
    drop(audio);
}

#[test]
fn sound_plays_on_dummy_audio_device() {
    let _guard = lock_sdl();
    std::env::set_var("SDL_AUDIODRIVER", "dummy");
    let mut audio =
        AudioSystem::init(MixerConfig::MONO).expect("o driver de áudio dummy deve abrir");
    assert_eq!(audio.allocate_channels(2), 2);
    assert_eq!(audio.spec().channels, 1);

    let tone = giochi_engine::audio::Sound::tone(440.0, 0.1, audio.spec().frequency)
        .expect("o tom deve ser sintetizado");
    tone.play().expect("o tom deve ser enfileirado no mixer");
    assert!(audio.playing_channels() >= 0);
}

#[test]
fn resource_manager_reports_missing_resources() {
    let _guard = lock_sdl();
    std::env::set_var("SDL_VIDEODRIVER", "dummy");
    let missing = std::path::Path::new("/caminho/que/nao/existe.png");
    let sdl = init_sdl().expect("o driver SDL2 dummy deve inicializar");
    let mut window =
        Window::new(&sdl, "recurso ausente", 16, 16).expect("o driver dummy deve criar uma janela");
    let (_context, loader) = window
        .render_context_with_loader()
        .expect("o contexto e o loader devem ser criados");
    let mut resources = ResourceManager::new();

    assert!(matches!(
        resources.texture(&loader, missing),
        Err(AssetError::InvalidPath { .. })
    ));
    assert_eq!(
        resources.stats().loads(),
        1,
        "a falta também conta como tentativa"
    );
    assert!(resources.is_empty());
}

#[test]
fn font_loader_reports_missing_font_file() {
    let _guard = lock_sdl();
    let loader = FontLoader::new().expect("o SDL_ttf deve inicializar");
    assert!(matches!(
        loader.load("/caminho/que/nao/existe.ttf", 16),
        Err(AssetError::InvalidPath { .. })
    ));
}

#[derive(Default)]
struct SceneStats {
    enters: usize,
    updates: usize,
    renders: usize,
    exits: usize,
}

/// Cena de teste que registra quantas chamadas do ciclo de vida recebeu.
struct RecordingScene {
    stats: std::rc::Rc<std::cell::RefCell<SceneStats>>,
    seconds: f32,
}

impl Scene for RecordingScene {
    fn on_enter(&mut self, _ctx: &mut SceneContext<'_, '_>) {
        self.stats.borrow_mut().enters += 1;
    }

    fn on_exit(&mut self) {
        self.stats.borrow_mut().exits += 1;
    }

    fn update(&mut self, dt: f32, _input: &Input) {
        self.seconds += dt;
        self.stats.borrow_mut().updates += 1;
    }

    fn render(&mut self, ctx: &mut RenderContext<'_>) -> Result<(), RenderError> {
        ctx.clear(Color::rgb(10, 20, 30));
        self.stats.borrow_mut().renders += 1;
        Ok(())
    }
}

#[test]
fn engine_steps_initial_scene_through_lifecycle() {
    let _guard = lock_sdl();
    std::env::set_var("SDL_VIDEODRIVER", "dummy");
    let sdl = init_sdl().expect("o driver SDL2 dummy deve inicializar");
    let mut window =
        Window::new(&sdl, "engine headless", 64, 48).expect("o driver dummy deve criar uma janela");
    let mut engine = Engine::new(
        sdl,
        &mut window,
        EngineConfig::new("engine", 64, 48).with_audio(false),
    )
    .expect("a engine deve montar");

    let stats = std::rc::Rc::new(std::cell::RefCell::new(SceneStats::default()));
    engine.scenes_mut().push(Box::new(RecordingScene {
        stats: std::rc::Rc::clone(&stats),
        seconds: 0.0,
    }));

    let running = engine.step(0.016).expect("um quadro deve ser desenhado");
    assert!(running);
    assert_eq!(
        stats.borrow().enters,
        1,
        "on_enter deve rodar no primeiro quadro"
    );
    assert_eq!(stats.borrow().updates, 1);
    assert_eq!(stats.borrow().renders, 1);

    engine.step(0.016).expect("segundo quadro");
    assert_eq!(stats.borrow().renders, 2);
    assert_eq!(stats.borrow().enters, 1, "on_enter não deve repetir");
}

#[test]
fn engine_replace_transition_calls_exit_and_enter() {
    let _guard = lock_sdl();
    std::env::set_var("SDL_VIDEODRIVER", "dummy");
    let sdl = init_sdl().expect("o driver SDL2 dummy deve inicializar");
    let mut window =
        Window::new(&sdl, "engine replace", 64, 48).expect("o driver dummy deve criar uma janela");
    let mut engine = Engine::new(
        sdl,
        &mut window,
        EngineConfig::new("engine", 64, 48).with_audio(false),
    )
    .expect("a engine deve montar");

    let first = std::rc::Rc::new(std::cell::RefCell::new(SceneStats::default()));
    let second = std::rc::Rc::new(std::cell::RefCell::new(SceneStats::default()));
    engine.scenes_mut().push(Box::new(RecordingScene {
        stats: std::rc::Rc::clone(&first),
        seconds: 0.0,
    }));
    engine.step(0.016).expect("primeiro quadro");

    engine.scenes_mut().replace(Box::new(RecordingScene {
        stats: std::rc::Rc::clone(&second),
        seconds: 0.0,
    }));
    engine.step(0.016).expect("quadro após a troca");

    assert_eq!(first.borrow().exits, 1, "a cena substituída recebe on_exit");
    assert_eq!(
        second.borrow().enters,
        1,
        "a nova cena recebe on_enter ao fim do quadro"
    );
    assert_eq!(
        second.borrow().renders,
        0,
        "a transição é aplicada depois do desenho, então ela desenha no quadro seguinte"
    );

    engine.step(0.016).expect("quadro seguinte");
    assert_eq!(second.borrow().renders, 1);
    assert_eq!(engine.scenes().depth(), 1);
}

#[test]
fn engine_run_exits_when_escape_is_queued() {
    let _guard = lock_sdl();
    std::env::set_var("SDL_VIDEODRIVER", "dummy");
    let sdl = init_sdl().expect("o driver SDL2 dummy deve inicializar");
    let mut window =
        Window::new(&sdl, "engine escape", 32, 32).expect("o driver dummy deve criar uma janela");
    let sdl_ctx = sdl.ctx.clone();
    let engine = Engine::new(
        sdl,
        &mut window,
        EngineConfig::new("engine", 32, 32).with_audio(false),
    )
    .expect("a engine deve montar");

    sdl_ctx
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

    let stats = std::rc::Rc::new(std::cell::RefCell::new(SceneStats::default()));
    engine
        .run(Box::new(RecordingScene {
            stats: std::rc::Rc::clone(&stats),
            seconds: 0.0,
        }))
        .expect("a engine deve encerrar sem erro");

    assert_eq!(
        stats.borrow().renders,
        0,
        "Escape encerra antes do primeiro desenho"
    );
}
