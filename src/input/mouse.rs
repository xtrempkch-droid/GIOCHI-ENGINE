//! Estado de mouse portátil: posição, botões e roda de rolagem.
//! Status: posição, transições de botões e rolagem implementadas via eventos SDL2.

use std::collections::HashSet;

use sdl2::{event::Event, mouse::MouseButton as SdlMouseButton};

/// Botões de mouse reconhecidos pela engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Botão principal.
    Left,
    /// Botão central (roda).
    Middle,
    /// Botão secundário.
    Right,
    /// Botão lateral adicional 1.
    X1,
    /// Botão lateral adicional 2.
    X2,
}

impl MouseButton {
    fn from_sdl(button: SdlMouseButton) -> Option<Self> {
        match button {
            SdlMouseButton::Left => Some(Self::Left),
            SdlMouseButton::Middle => Some(Self::Middle),
            SdlMouseButton::Right => Some(Self::Right),
            SdlMouseButton::X1 => Some(Self::X1),
            SdlMouseButton::X2 => Some(Self::X2),
            SdlMouseButton::Unknown => None,
        }
    }
}

/// Estado do mouse em coordenadas de janela.
#[derive(Debug, Default, Clone)]
pub struct MouseState {
    position: (i32, i32),
    wheel: (i32, i32),
    down: HashSet<MouseButton>,
    pressed: HashSet<MouseButton>,
    released: HashSet<MouseButton>,
}

impl MouseState {
    /// Limpa as transições e a rolagem do quadro anterior.
    pub fn begin_frame(&mut self) {
        self.pressed.clear();
        self.released.clear();
        self.wheel = (0, 0);
    }

    /// Aplica um evento SDL2 de mouse ao estado.
    pub fn apply_event(&mut self, event: &Event) {
        match event {
            Event::MouseMotion { x, y, .. } => self.position = (*x, *y),
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                self.position = (*x, *y);
                if let Some(button) = MouseButton::from_sdl(*mouse_btn) {
                    if self.down.insert(button) {
                        self.pressed.insert(button);
                    }
                }
            }
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                self.position = (*x, *y);
                if let Some(button) = MouseButton::from_sdl(*mouse_btn) {
                    if self.down.remove(&button) {
                        self.released.insert(button);
                    }
                }
            }
            Event::MouseWheel { x, y, .. } => {
                self.wheel.0 += x;
                self.wheel.1 += y;
            }
            _ => {}
        }
    }

    /// Posição atual do ponteiro em coordenadas de janela.
    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    /// Deslocamento acumulado da roda no quadro, em passos inteiros.
    pub fn wheel(&self) -> (i32, i32) {
        self.wheel
    }

    /// Informa se o botão está pressionado neste momento.
    pub fn is_down(&self, button: MouseButton) -> bool {
        self.down.contains(&button)
    }

    /// Informa se o botão foi pressionado neste quadro.
    pub fn is_pressed_this_frame(&self, button: MouseButton) -> bool {
        self.pressed.contains(&button)
    }

    /// Informa se o botão foi liberado neste quadro.
    pub fn is_released_this_frame(&self, button: MouseButton) -> bool {
        self.released.contains(&button)
    }

    /// Atualiza a posição sem gerar transições, útil antes do primeiro evento.
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    /// Esquece todo o estado, inclusive botões mantidos.
    pub fn clear(&mut self) {
        self.position = (0, 0);
        self.wheel = (0, 0);
        self.down.clear();
        self.pressed.clear();
        self.released.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn button_down(button: SdlMouseButton, x: i32, y: i32) -> Event {
        Event::MouseButtonDown {
            timestamp: 0,
            window_id: 0,
            which: 0,
            mouse_btn: button,
            clicks: 1,
            x,
            y,
        }
    }

    fn button_up(button: SdlMouseButton, x: i32, y: i32) -> Event {
        Event::MouseButtonUp {
            timestamp: 0,
            window_id: 0,
            which: 0,
            mouse_btn: button,
            clicks: 1,
            x,
            y,
        }
    }

    #[test]
    fn tracks_mouse_position_and_button_transitions() {
        let mut mouse = MouseState::default();
        mouse.apply_event(&button_down(SdlMouseButton::Right, 120, 80));

        assert_eq!(mouse.position(), (120, 80));
        assert!(mouse.is_down(MouseButton::Right));
        assert!(mouse.is_pressed_this_frame(MouseButton::Right));

        mouse.begin_frame();
        assert!(mouse.is_down(MouseButton::Right));
        assert!(!mouse.is_pressed_this_frame(MouseButton::Right));

        mouse.apply_event(&button_up(SdlMouseButton::Right, 130, 90));
        assert_eq!(mouse.position(), (130, 90));
        assert!(!mouse.is_down(MouseButton::Right));
        assert!(mouse.is_released_this_frame(MouseButton::Right));
    }

    #[test]
    fn accumulates_wheel_motion_per_frame() {
        let mut mouse = MouseState::default();
        mouse.apply_event(&Event::MouseWheel {
            timestamp: 0,
            window_id: 0,
            which: 0,
            x: 0,
            y: 1,
            direction: sdl2::mouse::MouseWheelDirection::Normal,
            precise_x: 0.0,
            precise_y: 1.0,
            mouse_x: 0,
            mouse_y: 0,
        });

        assert_eq!(mouse.wheel(), (0, 1));
        mouse.begin_frame();
        assert_eq!(mouse.wheel(), (0, 0));
    }
}
