//! Estado de teclado consultável por quadro, independente do jogo.
//! Status: pressionado, mantido e liberado implementados com eventos SDL2.

use std::collections::HashSet;

use sdl2::{event::Event, keyboard::Keycode};

use crate::math::vec2::Vec2;

/// Estado do teclado com transições válidas durante um único quadro.
///
/// `pressed` e `released` descrevem apenas o quadro atual; `down` acompanha o
/// estado contínuo. O loop chama `begin_frame` antes de aplicar novos eventos.
#[derive(Debug, Default, Clone)]
pub struct KeyboardState {
    down: HashSet<Keycode>,
    pressed: HashSet<Keycode>,
    released: HashSet<Keycode>,
}

impl KeyboardState {
    /// Limpa as transições do quadro anterior, preservando as teclas mantidas.
    pub fn begin_frame(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    /// Aplica um evento SDL2 de teclado ao estado.
    ///
    /// Eventos repetidos de `KeyDown` não geram uma nova transição de pressionado.
    pub fn apply_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key),
                repeat: false,
                ..
            } => {
                let is_new_press = self.down.insert(*key);
                if is_new_press {
                    self.pressed.insert(*key);
                }
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                let was_down = self.down.remove(key);
                if was_down {
                    self.released.insert(*key);
                }
            }
            _ => {}
        }
    }

    /// Informa se a tecla está pressionada neste momento.
    pub fn is_down(&self, key: Keycode) -> bool {
        self.down.contains(&key)
    }

    /// Informa se a tecla passou de liberada para pressionada neste quadro.
    pub fn is_pressed_this_frame(&self, key: Keycode) -> bool {
        self.pressed.contains(&key)
    }

    /// Informa se a tecla passou de pressionada para liberada neste quadro.
    pub fn is_released_this_frame(&self, key: Keycode) -> bool {
        self.released.contains(&key)
    }

    /// Itera sobre as teclas que começaram a ser pressionadas neste quadro.
    pub fn pressed(&self) -> impl Iterator<Item = Keycode> + '_ {
        self.pressed.iter().copied()
    }

    /// Itera sobre as teclas liberadas neste quadro.
    pub fn released(&self) -> impl Iterator<Item = Keycode> + '_ {
        self.released.iter().copied()
    }

    /// Esquece todo o estado, inclusive teclas mantidas.
    pub fn clear(&mut self) {
        self.down.clear();
        self.pressed.clear();
        self.released.clear();
    }

    /// Direção normalizada obtida das setas e de WASD.
    ///
    /// Combinações diagonais são normalizadas, mantendo a velocidade constante.
    pub fn movement(&self) -> Vec2 {
        let mut direction = Vec2::ZERO;
        if self.is_down(Keycode::Left) || self.is_down(Keycode::A) {
            direction.x -= 1.0;
        }
        if self.is_down(Keycode::Right) || self.is_down(Keycode::D) {
            direction.x += 1.0;
        }
        if self.is_down(Keycode::Up) || self.is_down(Keycode::W) {
            direction.y -= 1.0;
        }
        if self.is_down(Keycode::Down) || self.is_down(Keycode::S) {
            direction.y += 1.0;
        }
        direction.normalize_or_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::keyboard::Mod;

    fn key_down(key: Keycode) -> Event {
        Event::KeyDown {
            timestamp: 0,
            window_id: 0,
            keycode: Some(key),
            scancode: None,
            keymod: Mod::NOMOD,
            repeat: false,
        }
    }

    fn key_up(key: Keycode) -> Event {
        Event::KeyUp {
            timestamp: 0,
            window_id: 0,
            keycode: Some(key),
            scancode: None,
            keymod: Mod::NOMOD,
            repeat: false,
        }
    }

    #[test]
    fn follows_pressed_held_released_lifecycle() {
        let mut keyboard = KeyboardState::default();

        keyboard.apply_event(&key_down(Keycode::Space));
        assert!(keyboard.is_down(Keycode::Space));
        assert!(keyboard.is_pressed_this_frame(Keycode::Space));
        assert!(!keyboard.is_released_this_frame(Keycode::Space));

        keyboard.begin_frame();
        assert!(keyboard.is_down(Keycode::Space));
        assert!(!keyboard.is_pressed_this_frame(Keycode::Space));

        keyboard.apply_event(&key_up(Keycode::Space));
        assert!(!keyboard.is_down(Keycode::Space));
        assert!(keyboard.is_released_this_frame(Keycode::Space));

        keyboard.begin_frame();
        assert!(!keyboard.is_released_this_frame(Keycode::Space));
    }

    #[test]
    fn ignores_repeated_key_down_events() {
        let mut keyboard = KeyboardState::default();
        keyboard.apply_event(&key_down(Keycode::A));
        keyboard.begin_frame();
        keyboard.apply_event(&Event::KeyDown {
            timestamp: 0,
            window_id: 0,
            keycode: Some(Keycode::A),
            scancode: None,
            keymod: Mod::NOMOD,
            repeat: true,
        });

        assert!(keyboard.is_down(Keycode::A));
        assert!(!keyboard.is_pressed_this_frame(Keycode::A));
    }

    #[test]
    fn movement_combines_arrows_and_wasd_and_normalizes() {
        let mut keyboard = KeyboardState::default();
        keyboard.apply_event(&key_down(Keycode::Right));
        keyboard.apply_event(&key_down(Keycode::W));

        let direction = keyboard.movement();
        assert!((direction.length() - 1.0).abs() < f32::EPSILON);
        assert!(direction.x > 0.0);
        assert!(direction.y < 0.0);
    }
}
