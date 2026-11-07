//! Estado de gamepad com botões e eixos normalizados.
//! Status: abertura do primeiro controle, botões, sticks e gatilhos implementados.

use std::collections::HashSet;

use sdl2::{
    controller::{Axis, Button, GameController},
    event::Event,
    GameControllerSubsystem,
};

use crate::math::vec2::Vec2;

/// Zona morta padrão aplicada aos eixos analógicos.
pub const DEFAULT_DEADZONE: f32 = 0.2;

/// Botões de gamepad expostos pela engine com nomes neutros de plataforma.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    /// Botão inferior (A no Xbox, cruz no DualShock).
    South,
    /// Botão direito (B no Xbox, círculo no DualShock).
    East,
    /// Botão esquerdo (X no Xbox, quadrado no DualShock).
    West,
    /// Botão superior (Y no Xbox, triângulo no DualShock).
    North,
    /// Botão de voltar.
    Back,
    /// Botão de menu.
    Start,
    /// Botão central do sistema.
    Guide,
    /// Clique do analógico esquerdo.
    LeftStick,
    /// Clique do analógico direito.
    RightStick,
    /// Gatilho/bumper esquerdo.
    LeftShoulder,
    /// Gatilho/bumper direito.
    RightShoulder,
    /// Direcional para cima.
    DpadUp,
    /// Direcional para baixo.
    DpadDown,
    /// Direcional para a esquerda.
    DpadLeft,
    /// Direcional para a direita.
    DpadRight,
}

impl GamepadButton {
    fn from_sdl(button: Button) -> Option<Self> {
        match button {
            Button::A => Some(Self::South),
            Button::B => Some(Self::East),
            Button::X => Some(Self::West),
            Button::Y => Some(Self::North),
            Button::Back => Some(Self::Back),
            Button::Start => Some(Self::Start),
            Button::Guide => Some(Self::Guide),
            Button::LeftStick => Some(Self::LeftStick),
            Button::RightStick => Some(Self::RightStick),
            Button::LeftShoulder => Some(Self::LeftShoulder),
            Button::RightShoulder => Some(Self::RightShoulder),
            Button::DPadUp => Some(Self::DpadUp),
            Button::DPadDown => Some(Self::DpadDown),
            Button::DPadLeft => Some(Self::DpadLeft),
            Button::DPadRight => Some(Self::DpadRight),
            _ => None,
        }
    }
}

/// Estado do primeiro gamepad conectado, com degradação silenciosa quando ausente.
pub struct GamepadState {
    // O controle deve ser liberado antes do subsistema que o abriu.
    controller: Option<GameController>,
    subsystem: Option<GameControllerSubsystem>,
    down: HashSet<GamepadButton>,
    pressed: HashSet<GamepadButton>,
    released: HashSet<GamepadButton>,
    left_stick: Vec2,
    right_stick: Vec2,
    left_trigger: f32,
    right_trigger: f32,
    deadzone: f32,
}

impl Default for GamepadState {
    fn default() -> Self {
        Self {
            controller: None,
            subsystem: None,
            down: HashSet::new(),
            pressed: HashSet::new(),
            released: HashSet::new(),
            left_stick: Vec2::ZERO,
            right_stick: Vec2::ZERO,
            left_trigger: 0.0,
            right_trigger: 0.0,
            deadzone: DEFAULT_DEADZONE,
        }
    }
}

impl GamepadState {
    /// Cria o estado e tenta abrir o primeiro controle disponível.
    ///
    /// A ausência do subsistema não é tratada como erro: a engine continua sem gamepad.
    pub fn new(ctx: &sdl2::Sdl) -> Self {
        let mut state = Self::default();
        match ctx.game_controller() {
            Ok(subsystem) => {
                state.subsystem = Some(subsystem);
                state.open_first_available();
            }
            Err(error) => {
                log::warn!("subsistema de gamepad indisponível: {error}");
            }
        }
        state
    }

    /// Limpa as transições do quadro anterior, preservando botões e eixos mantidos.
    pub fn begin_frame(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    /// Aplica um evento SDL2 de controle ao estado.
    pub fn apply_event(&mut self, event: &Event) {
        match event {
            Event::ControllerDeviceAdded { which, .. } => {
                if self.controller.is_none() {
                    self.open(*which);
                }
            }
            Event::ControllerDeviceRemoved { which, .. } => {
                if self
                    .controller
                    .as_ref()
                    .is_some_and(|controller| controller.instance_id() == *which)
                {
                    log::info!("gamepad desconectado");
                    self.controller = None;
                    self.down.clear();
                    self.pressed.clear();
                    self.released.clear();
                    self.left_stick = Vec2::ZERO;
                    self.right_stick = Vec2::ZERO;
                    self.left_trigger = 0.0;
                    self.right_trigger = 0.0;
                }
            }
            Event::ControllerButtonDown { button, .. } => {
                if let Some(button) = GamepadButton::from_sdl(*button) {
                    if self.down.insert(button) {
                        self.pressed.insert(button);
                    }
                }
            }
            Event::ControllerButtonUp { button, .. } => {
                if let Some(button) = GamepadButton::from_sdl(*button) {
                    if self.down.remove(&button) {
                        self.released.insert(button);
                    }
                }
            }
            Event::ControllerAxisMotion { axis, value, .. } => {
                self.apply_axis(*axis, *value);
            }
            _ => {}
        }
    }

    /// Informa se um controle está aberto.
    pub fn is_connected(&self) -> bool {
        self.controller.is_some()
    }

    /// Nome do controle conectado, quando houver.
    pub fn name(&self) -> Option<String> {
        self.controller.as_ref().map(GameController::name)
    }

    /// Informa se o botão está pressionado neste momento.
    pub fn is_down(&self, button: GamepadButton) -> bool {
        self.down.contains(&button)
    }

    /// Informa se o botão foi pressionado neste quadro.
    pub fn is_pressed_this_frame(&self, button: GamepadButton) -> bool {
        self.pressed.contains(&button)
    }

    /// Informa se o botão foi liberado neste quadro.
    pub fn is_released_this_frame(&self, button: GamepadButton) -> bool {
        self.released.contains(&button)
    }

    /// Posição do analógico esquerdo, já com a zona morta aplicada.
    pub fn left_stick(&self) -> Vec2 {
        self.left_stick
    }

    /// Posição do analógico direito, já com a zona morta aplicada.
    pub fn right_stick(&self) -> Vec2 {
        self.right_stick
    }

    /// Valores normalizados dos gatilhos esquerdo e direito em `0.0..=1.0`.
    pub fn triggers(&self) -> (f32, f32) {
        (self.left_trigger, self.right_trigger)
    }

    /// Zona morta aplicada aos analógicos.
    pub fn deadzone(&self) -> f32 {
        self.deadzone
    }

    /// Define a zona morta dos analógicos, limitada ao intervalo `0.0..=1.0`.
    pub fn set_deadzone(&mut self, deadzone: f32) {
        self.deadzone = deadzone.clamp(0.0, 1.0);
    }

    fn open_first_available(&mut self) {
        let candidate = {
            let Some(subsystem) = self.subsystem.as_ref() else {
                return;
            };
            let count = match subsystem.num_joysticks() {
                Ok(count) => count,
                Err(error) => {
                    log::warn!("não foi possível enumerar joysticks: {error}");
                    return;
                }
            };
            (0..count).find(|index| subsystem.is_game_controller(*index))
        };
        if let Some(index) = candidate {
            self.open(index);
        }
    }

    fn open(&mut self, index: u32) {
        let Some(subsystem) = self.subsystem.as_ref() else {
            return;
        };
        match subsystem.open(index) {
            Ok(controller) => {
                log::info!("gamepad conectado: {}", controller.name());
                self.controller = Some(controller);
            }
            Err(error) => {
                log::warn!("não foi possível abrir o gamepad {index}: {error}");
            }
        }
    }

    fn apply_axis(&mut self, axis: Axis, value: i16) {
        let normalized = f32::from(value) / f32::from(i16::MAX);
        match axis {
            Axis::LeftX => self.left_stick.x = apply_deadzone(normalized, self.deadzone),
            Axis::LeftY => self.left_stick.y = apply_deadzone(normalized, self.deadzone),
            Axis::RightX => self.right_stick.x = apply_deadzone(normalized, self.deadzone),
            Axis::RightY => self.right_stick.y = apply_deadzone(normalized, self.deadzone),
            Axis::TriggerLeft => self.left_trigger = normalized.clamp(0.0, 1.0),
            Axis::TriggerRight => self.right_trigger = normalized.clamp(0.0, 1.0),
        }
    }
}

fn apply_deadzone(value: f32, deadzone: f32) -> f32 {
    if value.abs() < deadzone {
        0.0
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn button_down(button: Button) -> Event {
        Event::ControllerButtonDown {
            timestamp: 0,
            which: 0,
            button,
        }
    }

    fn button_up(button: Button) -> Event {
        Event::ControllerButtonUp {
            timestamp: 0,
            which: 0,
            button,
        }
    }

    #[test]
    fn maps_sdl_buttons_and_tracks_transitions() {
        let mut gamepad = GamepadState::default();

        gamepad.apply_event(&button_down(Button::A));
        assert!(gamepad.is_down(GamepadButton::South));
        assert!(gamepad.is_pressed_this_frame(GamepadButton::South));

        gamepad.begin_frame();
        assert!(gamepad.is_down(GamepadButton::South));
        assert!(!gamepad.is_pressed_this_frame(GamepadButton::South));

        gamepad.apply_event(&button_up(Button::A));
        assert!(!gamepad.is_down(GamepadButton::South));
        assert!(gamepad.is_released_this_frame(GamepadButton::South));
    }

    #[test]
    fn applies_deadzone_and_normalizes_axis_ranges() {
        let mut gamepad = GamepadState::default();

        gamepad.apply_event(&Event::ControllerAxisMotion {
            timestamp: 0,
            which: 0,
            axis: Axis::LeftX,
            value: 1000,
        });
        assert_eq!(gamepad.left_stick().x, 0.0);

        gamepad.apply_event(&Event::ControllerAxisMotion {
            timestamp: 0,
            which: 0,
            axis: Axis::LeftX,
            value: i16::MAX,
        });
        assert!((gamepad.left_stick().x - 1.0).abs() < f32::EPSILON);

        gamepad.apply_event(&Event::ControllerAxisMotion {
            timestamp: 0,
            which: 0,
            axis: Axis::TriggerRight,
            value: i16::MAX,
        });
        assert!((gamepad.triggers().1 - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn deadzone_is_clamped_to_valid_range() {
        let mut gamepad = GamepadState::default();
        gamepad.set_deadzone(2.0);
        assert!((gamepad.deadzone() - 1.0).abs() < f32::EPSILON);
        gamepad.set_deadzone(-1.0);
        assert_eq!(gamepad.deadzone(), 0.0);
    }
}
