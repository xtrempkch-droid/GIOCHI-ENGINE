//! Pong jogável — teste de fumaça da engine com todos os módulos.
//! Status: exemplo funcional de `Engine`, cenas, input, render, assets e áudio.
//!
//! Controles: W/S, setas ou analógico movem a raquete; Espaço inicia e reinicia.
//! A partida termina em 5 pontos. Os sons são WAV gerados em tempo de execução e
//! carregados pelo `ResourceManager`, exercitando o cache de assets; se o áudio
//! não estiver disponível, o jogo continua em silêncio.

use std::{cell::Cell, f32::consts::TAU, fs, io::Write, path::PathBuf, rc::Rc};

use anyhow::Result;
use giochi_engine::{
    assets::ResourceManager,
    audio::Sound,
    core::engine::{Engine, EngineConfig},
    input::{GamepadButton, Input},
    math::{color::Color, rect::Rect, vec2::Vec2},
    platform::{init_sdl, Window},
    render::{renderer::RenderContext, RenderError},
    scene::{Scene, SceneContext},
};
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const PADDLE_W: u32 = 14;
const PADDLE_H: u32 = 88;
const PADDLE_X: i32 = 32;
const BALL: u32 = 16;
const WIN_SCORE: u32 = 5;
const STEP: f32 = 1.0 / 60.0;

/// Sons compartilhados pelas cenas.
#[derive(Clone, Default)]
struct Sounds {
    hit: Option<Rc<Sound>>,
    score: Option<Rc<Sound>>,
}

impl Sounds {
    fn play(&self, sound: &Option<Rc<Sound>>) {
        if let Some(sound) = sound {
            if let Err(error) = sound.play() {
                log::debug!("falha ao reproduzir som: {error}");
            }
        }
    }

    fn play_hit(&self) {
        self.play(&self.hit);
    }

    fn play_score(&self) {
        self.play(&self.score);
    }
}

/// Tela inicial: instruções e espera por Espaço.
struct MenuScene {
    sounds: Sounds,
    elapsed: f32,
}

impl Scene for MenuScene {
    fn on_enter(&mut self, _ctx: &mut SceneContext<'_, '_>) {
        self.sounds.play_hit();
    }

    fn update(&mut self, dt: f32, _input: &Input) {
        self.elapsed += dt;
    }

    fn render(&mut self, context: &mut RenderContext<'_>) -> Result<(), RenderError> {
        context.clear(Color::rgb(14, 18, 28));
        // Faixa decorativa animada.
        let offset = ((self.elapsed * 60.0) as i32) % 64;
        context.draw_rect(Rect::new(offset - 64, 96, 48, 8), Color::rgb(90, 200, 240))?;
        context.draw_rect(Rect::new(272, 220, 96, 14), Color::rgb(240, 240, 240))?;
        context.draw_rect(Rect::new(300, 250, 40, 10), Color::rgb(150, 160, 180))?;
        context.draw_rect(Rect::new(240, 320, 160, 18), Color::rgb(90, 220, 160))?;
        Ok(())
    }
}

/// Partida de Pong com raquete do jogador, adversário simples e placar.
struct PongScene {
    sounds: Sounds,
    finished: Rc<Cell<bool>>,
    player_y: f32,
    cpu_y: f32,
    ball: Vec2,
    velocity: Vec2,
    score_player: u32,
    score_cpu: u32,
    flash: f32,
}

impl PongScene {
    fn new(sounds: Sounds, finished: Rc<Cell<bool>>) -> Self {
        Self {
            sounds,
            finished,
            player_y: HEIGHT as f32 / 2.0,
            cpu_y: HEIGHT as f32 / 2.0,
            ball: Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            velocity: Vec2::new(220.0, 130.0),
            score_player: 0,
            score_cpu: 0,
            flash: 0.0,
        }
    }

    fn player_rect(&self) -> Rect {
        Rect::new(
            PADDLE_X,
            (self.player_y - PADDLE_H as f32 / 2.0).round() as i32,
            PADDLE_W,
            PADDLE_H,
        )
    }

    fn cpu_rect(&self) -> Rect {
        Rect::new(
            WIDTH as i32 - PADDLE_X - PADDLE_W as i32,
            (self.cpu_y - PADDLE_H as f32 / 2.0).round() as i32,
            PADDLE_W,
            PADDLE_H,
        )
    }

    fn ball_rect(&self) -> Rect {
        Rect::new(
            (self.ball.x - BALL as f32 / 2.0).round() as i32,
            (self.ball.y - BALL as f32 / 2.0).round() as i32,
            BALL,
            BALL,
        )
    }

    fn reset_ball(&mut self, towards_player: bool) {
        self.ball = Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
        let direction = if towards_player { -1.0 } else { 1.0 };
        self.velocity = Vec2::new(220.0 * direction, 130.0);
    }

    fn score_point(&mut self, player: bool) {
        if player {
            self.score_player += 1;
        } else {
            self.score_cpu += 1;
        }
        self.flash = 0.35;
        self.sounds.play_score();
        self.reset_ball(!player);

        if self.score_player >= WIN_SCORE || self.score_cpu >= WIN_SCORE {
            self.finished.set(true);
        }
    }
}

impl Scene for PongScene {
    fn update(&mut self, dt: f32, input: &Input) {
        let direction = input.keyboard.movement().y + input.gamepad.left_stick().y;
        self.player_y = (self.player_y + direction * 320.0 * dt)
            .clamp(PADDLE_H as f32 / 2.0, HEIGHT as f32 - PADDLE_H as f32 / 2.0);
        self.cpu_y += (self.ball.y - self.cpu_y).clamp(-260.0 * dt, 260.0 * dt);

        self.ball += self.velocity * dt;
        self.flash = (self.flash - dt).max(0.0);

        let ball_rect = self.ball_rect();
        let half = BALL as f32 / 2.0;

        if ball_rect.y <= 0 {
            self.ball.y = half;
            self.velocity.y = self.velocity.y.abs();
            self.sounds.play_hit();
        } else if ball_rect.y + BALL as i32 >= HEIGHT as i32 {
            self.ball.y = HEIGHT as f32 - half;
            self.velocity.y = -self.velocity.y.abs();
            self.sounds.play_hit();
        }

        if self.velocity.x < 0.0 && ball_rect.intersects(self.player_rect()) {
            self.ball.x = (PADDLE_X + PADDLE_W as i32) as f32 + half;
            self.velocity.x = self.velocity.x.abs() * 1.05;
            self.velocity.y += direction * 40.0;
            self.sounds.play_hit();
        } else if self.velocity.x > 0.0 && ball_rect.intersects(self.cpu_rect()) {
            self.ball.x = (WIDTH as i32 - PADDLE_X - PADDLE_W as i32) as f32 - half;
            self.velocity.x = -self.velocity.x.abs() * 1.05;
            self.sounds.play_hit();
        }

        self.velocity = self.velocity.clamp_length_max(520.0);

        if self.ball.x < -half * 4.0 {
            self.score_point(false);
        } else if self.ball.x > WIDTH as f32 + half * 4.0 {
            self.score_point(true);
        }
    }

    fn render(&mut self, context: &mut RenderContext<'_>) -> Result<(), RenderError> {
        let background = if self.flash > 0.0 {
            Color::rgb(30, 40, 60)
        } else {
            Color::rgb(12, 16, 24)
        };
        context.clear(background);

        // Rede central tracejada.
        for y in (0..HEIGHT as i32).step_by(24) {
            context.draw_rect(
                Rect::new(WIDTH as i32 / 2 - 2, y, 4, 12),
                Color::rgb(50, 60, 78),
            )?;
        }

        context.draw_rect(self.player_rect(), Color::rgb(90, 220, 160))?;
        context.draw_rect(self.cpu_rect(), Color::rgb(240, 160, 90))?;
        context.draw_rect(self.ball_rect(), Color::rgb(245, 245, 245))?;

        // Placar em blocos: uma barra por ponto conquistado.
        for point in 0..self.score_player {
            context.draw_rect(
                Rect::new(60, 20 + point as i32 * 14, 18, 10),
                Color::rgb(90, 220, 160),
            )?;
        }
        for point in 0..self.score_cpu {
            context.draw_rect(
                Rect::new(WIDTH as i32 - 78, 20 + point as i32 * 14, 18, 10),
                Color::rgb(240, 160, 90),
            )?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let sdl = init_sdl()?;
    let config = EngineConfig::new("GIOCHI-ENGINE — Mini Game", WIDTH, HEIGHT);
    let mut window = Window::new(&sdl, &config.title, config.width, config.height)?;
    let mut engine = Engine::new(sdl, &mut window, config)?;

    let sounds = load_sounds(&mut engine);
    let finished = Rc::new(Cell::new(false));

    engine.scenes_mut().push(Box::new(MenuScene {
        sounds: sounds.clone(),
        elapsed: 0.0,
    }));

    let mut in_game = false;
    while engine.step(STEP)? {
        let start_pressed = engine
            .input()
            .keyboard
            .is_pressed_this_frame(Keycode::Space)
            || engine
                .input()
                .gamepad
                .is_pressed_this_frame(GamepadButton::South);

        if !in_game && start_pressed {
            in_game = true;
            engine.scenes_mut().replace(Box::new(PongScene::new(
                sounds.clone(),
                Rc::clone(&finished),
            )));
        } else if in_game && finished.get() {
            in_game = false;
            finished.set(false);
            engine.scenes_mut().replace(Box::new(MenuScene {
                sounds: sounds.clone(),
                elapsed: 0.0,
            }));
        }
    }
    Ok(())
}

/// Gera dois WAV curtos em um diretório temporário e os carrega pelo cache.
fn load_sounds(engine: &mut Engine<'_>) -> Sounds {
    if engine.audio().is_none() {
        return Sounds::default();
    }
    let sample_rate = engine
        .audio()
        .map_or(44_100, |audio| audio.spec().frequency);
    let directory = std::env::temp_dir().join("giochi-mini-game");
    if let Err(error) = fs::create_dir_all(&directory) {
        log::warn!("não foi possível criar o diretório de sons: {error}");
        return fallback_sounds(sample_rate);
    }

    let hit_path = directory.join("hit.wav");
    let score_path = directory.join("score.wav");
    let mut sounds = Sounds::default();
    let mut resources = ResourceManager::new();

    if write_wave(&hit_path, 880.0, 0.08, sample_rate).is_ok() {
        match resources.sound(&hit_path) {
            Ok(sound) => sounds.hit = Some(sound),
            Err(error) => log::warn!("som de rebote indisponível: {error}"),
        }
    }
    if write_wave(&score_path, 320.0, 0.25, sample_rate).is_ok() {
        match resources.sound(&score_path) {
            Ok(sound) => sounds.score = Some(sound),
            Err(error) => log::warn!("som de ponto indisponível: {error}"),
        }
    }

    if sounds.hit.is_none() && sounds.score.is_none() {
        fallback_sounds(sample_rate)
    } else {
        log::info!(
            "assets carregados: {} sons em cache",
            resources.sound_count()
        );
        sounds
    }
}

fn fallback_sounds(sample_rate: i32) -> Sounds {
    Sounds {
        hit: Sound::tone(880.0, 0.08, sample_rate).ok().map(Rc::new),
        score: Sound::tone(320.0, 0.25, sample_rate).ok().map(Rc::new),
    }
}

/// Escreve um WAV PCM 16 bits mono com uma senoide curta.
fn write_wave(
    path: &PathBuf,
    frequency: f32,
    seconds: f32,
    sample_rate: i32,
) -> std::io::Result<()> {
    let sample_rate = sample_rate.max(8_000) as u32;
    let total = (sample_rate as f32 * seconds) as u32;
    let data_size = total * 2;
    let mut file = fs::File::create(path)?;
    file.write_all(b"RIFF")?;
    file.write_all(&(36 + data_size).to_le_bytes())?;
    file.write_all(b"WAVEfmt ")?;
    file.write_all(&16_u32.to_le_bytes())?;
    file.write_all(&1_u16.to_le_bytes())?;
    file.write_all(&1_u16.to_le_bytes())?;
    file.write_all(&sample_rate.to_le_bytes())?;
    file.write_all(&(sample_rate * 2).to_le_bytes())?;
    file.write_all(&2_u16.to_le_bytes())?;
    file.write_all(&16_u16.to_le_bytes())?;
    file.write_all(b"data")?;
    file.write_all(&data_size.to_le_bytes())?;
    for index in 0..total {
        let phase = TAU * frequency * index as f32 / sample_rate as f32;
        let fade = 1.0 - index as f32 / total as f32;
        let sample = (phase.sin() * fade * 12_000.0) as i16;
        file.write_all(&sample.to_le_bytes())?;
    }
    Ok(())
}
