//! Medições de delta time e quadros por segundo.
//! Status: cálculos básicos implementados.

/// Estatísticas do quadro atual, em segundos e quadros por segundo.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Time {
    delta_seconds: f32,
    frames_per_second: f32,
}

impl Time {
    /// Cria as estatísticas a partir do delta em segundos.
    pub fn from_delta(delta_seconds: f32) -> Self {
        let frames_per_second = if delta_seconds > 0.0 {
            1.0 / delta_seconds
        } else {
            0.0
        };
        Self {
            delta_seconds,
            frames_per_second,
        }
    }

    /// Retorna o tempo do quadro anterior em segundos.
    pub fn delta_seconds(self) -> f32 {
        self.delta_seconds
    }

    /// Retorna a estimativa instantânea de quadros por segundo.
    pub fn frames_per_second(self) -> f32 {
        self.frames_per_second
    }
}
