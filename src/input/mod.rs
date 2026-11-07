//! Entrada unificada de teclado, mouse e gamepad para a engine.
//! Status: estados por quadro, decodificação SDL2 e gamepad opcional implementados.
//!
//! O jogo recebe apenas `Input`; a decodificação de eventos SDL2 permanece nesta
//! camada, permitindo substituir o backend sem alterar a lógica dos jogos.

pub mod gamepad;
pub mod keyboard;
pub mod mouse;

use sdl2::event::Event;

pub use gamepad::{GamepadButton, GamepadState};
pub use keyboard::KeyboardState;
pub use mouse::{MouseButton, MouseState};

/// Agregado de entrada consultado durante um quadro.
#[derive(Default)]
pub struct Input {
    /// Estado do teclado.
    pub keyboard: KeyboardState,
    /// Estado do mouse.
    pub mouse: MouseState,
    /// Estado do primeiro gamepad conectado, se houver.
    pub gamepad: GamepadState,
}

impl Input {
    /// Cria a entrada e tenta abrir o primeiro gamepad disponível.
    pub fn new(ctx: &sdl2::Sdl) -> Self {
        Self {
            keyboard: KeyboardState::default(),
            mouse: MouseState::default(),
            gamepad: GamepadState::new(ctx),
        }
    }

    /// Cria uma entrada sem tentar abrir o subsistema de gamepad.
    ///
    /// Útil para testes unitários e para consumidores que não desejam detectar controles.
    pub fn headless() -> Self {
        Self::default()
    }

    /// Limpa as transições do quadro anterior antes de aplicar novos eventos.
    pub fn begin_frame(&mut self) {
        self.keyboard.begin_frame();
        self.mouse.begin_frame();
        self.gamepad.begin_frame();
    }

    /// Aplica uma sequência de eventos SDL2 aos estados de entrada.
    pub fn update(&mut self, events: &[Event]) {
        for event in events {
            self.apply_event(event);
        }
    }

    /// Aplica um único evento SDL2 aos estados de entrada.
    pub fn apply_event(&mut self, event: &Event) {
        self.keyboard.apply_event(event);
        self.mouse.apply_event(event);
        self.gamepad.apply_event(event);
    }

    /// Esquece todo o estado, inclusive teclas, botões e eixos mantidos.
    pub fn clear(&mut self) {
        self.keyboard.clear();
        self.mouse.clear();
        self.gamepad = GamepadState::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::{
        keyboard::{Keycode, Mod},
        mouse::MouseButton as SdlMouseButton,
    };

    #[test]
    fn update_routes_events_to_every_device() {
        let mut input = Input::headless();
        input.update(&[
            Event::KeyDown {
                timestamp: 0,
                window_id: 0,
                keycode: Some(Keycode::D),
                scancode: None,
                keymod: Mod::NOMOD,
                repeat: false,
            },
            Event::MouseButtonDown {
                timestamp: 0,
                window_id: 0,
                which: 0,
                mouse_btn: SdlMouseButton::Left,
                clicks: 1,
                x: 10,
                y: 20,
            },
        ]);

        assert!(input.keyboard.is_down(Keycode::D));
        assert!(input.mouse.is_down(MouseButton::Left));
        assert_eq!(input.mouse.position(), (10, 20));
    }

    #[test]
    fn begin_frame_preserves_held_state_but_clears_transitions() {
        let mut input = Input::headless();
        input.update(&[Event::KeyDown {
            timestamp: 0,
            window_id: 0,
            keycode: Some(Keycode::Space),
            scancode: None,
            keymod: Mod::NOMOD,
            repeat: false,
        }]);
        input.begin_frame();

        assert!(input.keyboard.is_down(Keycode::Space));
        assert!(!input.keyboard.is_pressed_this_frame(Keycode::Space));
    }
}
