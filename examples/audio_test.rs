//! Toca dois tons sintetizados ao pressionar Espaço e Enter.
//! Status: exemplo funcional de AudioSystem, Sound procedural e entrada unificada.
//!
//! Nenhum arquivo de áudio é necessário: as amostras são geradas em memória.
//! Se o aparelho não tiver dispositivo de áudio, o exemplo cai para o driver
//! `dummy` do SDL2 e continua executando sem som.

use anyhow::Result;
use giochi_engine::{
    audio::{AudioSystem, MixerConfig, Sound},
    core::game_loop::{self, Game},
    input::{GamepadButton, Input},
    math::{color::Color, rect::Rect},
    platform::{init_sdl, Window},
    render::{renderer::RenderContext, RenderError},
};
use sdl2::keyboard::Keycode;

const LOW_TONE_HZ: f32 = 440.0;
const HIGH_TONE_HZ: f32 = 660.0;

struct AudioDemo {
    sounds: Option<(Sound, Sound)>,
    last_tone: usize,
    pulse: f32,
}

impl AudioDemo {
    fn trigger(&mut self, index: usize) {
        self.last_tone = index;
        self.pulse = 1.0;
        if let Some(sounds) = &self.sounds {
            let sound = if index == 0 { &sounds.0 } else { &sounds.1 };
            if let Err(error) = sound.play() {
                log::warn!("falha ao tocar o tom: {error}");
            }
        }
    }
}

impl Game for AudioDemo {
    fn update(&mut self, dt: f32, input: &Input) {
        let low = input.keyboard.is_pressed_this_frame(Keycode::Space)
            || input.gamepad.is_pressed_this_frame(GamepadButton::South);
        if low {
            self.trigger(0);
        }

        let high = input.keyboard.is_pressed_this_frame(Keycode::Return)
            || input.gamepad.is_pressed_this_frame(GamepadButton::East);
        if high {
            self.trigger(1);
        }

        self.pulse = (self.pulse - dt * 2.5).max(0.0);
    }

    fn render(&mut self, context: &mut RenderContext<'_>) -> Result<(), RenderError> {
        context.clear(Color::rgb(12, 16, 24));
        let active = usize::from(self.last_tone != 0);
        for index in [0_i32, 1] {
            let color = if index as usize == active {
                Color::rgb(90, 220, 170)
            } else {
                Color::rgb(60, 70, 90)
            };
            context.draw_rect(Rect::new(240 + index * 110, 200, 80, 80), color)?;
        }
        let bar_height = (self.pulse * 120.0) as u32;
        context.draw_rect(
            Rect::new(
                320 - (bar_height as i32 / 2),
                380 - bar_height as i32,
                bar_height,
                12,
            ),
            Color::rgb(245, 200, 90),
        )?;
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let mut sdl = init_sdl()?;
    let mut window = Window::new(&sdl, "GIOCHI-ENGINE — Áudio", 640, 480)?;

    let mut audio = open_audio();
    if let Some(system) = audio.as_mut() {
        system.allocate_channels(4);
    }
    let sounds = audio.as_ref().map(|system| {
        // O buffer fornecido precisa estar no formato negociado com o backend.
        let sample_rate = system.spec().frequency;
        (
            Sound::tone(LOW_TONE_HZ, 0.35, sample_rate),
            Sound::tone(HIGH_TONE_HZ, 0.35, sample_rate),
        )
    });
    let sounds = match sounds {
        Some((Ok(low), Ok(high))) => Some((low, high)),
        Some((low, high)) => {
            if let Err(error) = low {
                log::warn!("tom grave indisponível: {error}");
            }
            if let Err(error) = high {
                log::warn!("tom agudo indisponível: {error}");
            }
            None
        }
        None => None,
    };

    let mut game = AudioDemo {
        sounds,
        last_tone: 0,
        pulse: 0.0,
    };
    let result = game_loop::run(&mut game, &mut window, &mut sdl);
    drop(audio);
    result
}

fn open_audio() -> Option<AudioSystem> {
    match AudioSystem::init(MixerConfig::MONO) {
        Ok(system) => Some(system),
        Err(error) => {
            log::warn!("{error}");
            log::warn!("tentando o driver de áudio descartável do SDL2");
            std::env::set_var("SDL_AUDIODRIVER", "dummy");
            match AudioSystem::init(MixerConfig::MONO) {
                Ok(system) => Some(system),
                Err(error) => {
                    log::warn!("{error}; o exemplo seguirá sem som");
                    None
                }
            }
        }
    }
}
