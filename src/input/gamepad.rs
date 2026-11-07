//! Identificação e estado mínimo de um controle.
//! Status: tipo de eixo básico; descoberta e leitura de controles ainda não implementadas.

/// Estado de um eixo analógico normalizado no intervalo de -1 a 1.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GamepadAxis {
    /// Valor horizontal normalizado.
    pub x: f32,
    /// Valor vertical normalizado.
    pub y: f32,
}
