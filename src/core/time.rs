//! Medições de delta time e quadros por segundo.
//! Status: cálculos básicos implementados.

use std::time::Instant;

/// Estatísticas acumuladas de tempo e quadros da aplicação.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    /// Tempo do quadro anterior, em segundos e limitado a 250 ms.
    pub delta: f32,
    /// Tempo total acumulado, em segundos.
    pub elapsed: f32,
    /// Quantidade de quadros medidos.
    pub frame_count: u64,
    frames_per_second: f32,
    last_tick: Instant,
}

impl Time {
    /// Cria um relógio cuja primeira medição começa em `now`.
    pub fn new(now: Instant) -> Self {
        Self {
            delta: 0.0,
            elapsed: 0.0,
            frame_count: 0,
            frames_per_second: 0.0,
            last_tick: now,
        }
    }

    /// Cria as estatísticas a partir do delta em segundos.
    pub fn from_delta(delta_seconds: f32) -> Self {
        let frames_per_second = if delta_seconds > 0.0 {
            1.0 / delta_seconds
        } else {
            0.0
        };
        Self {
            delta: delta_seconds,
            elapsed: 0.0,
            frame_count: 0,
            frames_per_second,
            last_tick: Instant::now(),
        }
    }

    /// Atualiza delta, tempo decorrido e contagem a partir do instante fornecido.
    ///
    /// Deltas acima de 250 ms são limitados para evitar que uma pausa longa provoque
    /// uma sequência excessiva de atualizações no loop.
    pub fn tick(&mut self, now: Instant) -> f32 {
        let delta = now
            .saturating_duration_since(self.last_tick)
            .as_secs_f32()
            .min(0.25);
        self.last_tick = now;
        self.delta = delta;
        self.elapsed += delta;
        self.frame_count += 1;
        self.frames_per_second = if delta > 0.0 { 1.0 / delta } else { 0.0 };
        delta
    }

    /// Retorna o tempo do quadro anterior em segundos.
    pub fn delta_seconds(self) -> f32 {
        self.delta
    }

    /// Retorna a estimativa instantânea de quadros por segundo.
    pub fn frames_per_second(self) -> f32 {
        self.frames_per_second
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new(Instant::now())
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::Time;

    #[test]
    fn tick_tracks_delta_elapsed_and_frame_count() {
        let start = Instant::now();
        let mut time = Time::new(start);

        time.tick(start + Duration::from_millis(100));

        assert!((time.delta - 0.1).abs() < 0.0001);
        assert!((time.elapsed - 0.1).abs() < 0.0001);
        assert_eq!(time.frame_count, 1);
        assert!((time.frames_per_second() - 10.0).abs() < 0.01);
    }

    #[test]
    fn tick_clamps_long_frames_to_quarter_second() {
        let start = Instant::now();
        let mut time = Time::new(start);

        time.tick(start + Duration::from_millis(800));

        assert_eq!(time.delta, 0.25);
        assert_eq!(time.elapsed, 0.25);
    }
}
