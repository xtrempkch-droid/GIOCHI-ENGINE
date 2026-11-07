//! Representação portátil da posição e dos botões do mouse.
//! Status: estrutura básica implementada, sem captura de eventos.

/// Estado capturado do mouse em coordenadas de janela.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MouseState {
    /// Posição horizontal.
    pub x: i32,
    /// Posição vertical.
    pub y: i32,
    /// Máscara dos botões pressionados.
    pub buttons: u32,
}
