//! Estado lógico de teclas independente do backend.
//! Status: estrutura básica implementada.

/// Teclas direcionais usadas pelo demo e por jogos 2D.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardState {
    /// Seta para cima pressionada.
    pub up: bool,
    /// Seta para baixo pressionada.
    pub down: bool,
    /// Seta para a esquerda pressionada.
    pub left: bool,
    /// Seta para a direita pressionada.
    pub right: bool,
}
