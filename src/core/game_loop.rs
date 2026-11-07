//! Loop de jogo com passo fixo, acumulador e limite de tempo por quadro.
//! Status: implementação funcional com callback de eventos.

use std::time::{Duration, Instant};

use crate::platform::window::RenderContext;

/// Passo fixo de simulação, equivalente a 60 atualizações por segundo.
pub const FIXED_DT: f32 = 1.0 / 60.0;
/// Tempo máximo incorporado ao acumulador em um quadro.
pub const MAX_FRAME_TIME: f64 = 0.25;

/// Contrato mínimo implementado por um jogo controlado pela engine.
pub trait Game {
    /// Atualiza a simulação em segundos.
    fn update(&mut self, dt: f32);
    /// Desenha um quadro usando a abstração de renderização da plataforma.
    fn render(&mut self, ctx: &mut RenderContext<'_>);
}

/// Executa o jogo até `continue_running` retornar `false`.
///
/// O callback é chamado uma vez por quadro para processar eventos da plataforma.
pub fn run<G, F>(game: &mut G, ctx: &mut RenderContext<'_>, mut continue_running: F)
where
    G: Game,
    F: FnMut(&mut G) -> bool,
{
    let fixed_dt = f64::from(FIXED_DT);
    let mut previous = Instant::now();
    let mut accumulator = 0.0_f64;
    let mut next_frame = Instant::now();

    while continue_running(game) {
        let now = Instant::now();
        let frame_time = (now - previous).as_secs_f64().min(MAX_FRAME_TIME);
        previous = now;
        accumulator += frame_time;

        while accumulator >= fixed_dt {
            game.update(FIXED_DT);
            accumulator -= fixed_dt;
        }

        game.render(ctx);
        next_frame += Duration::from_secs_f64(fixed_dt);
        let now = Instant::now();
        if next_frame > now {
            std::thread::sleep(next_frame - now);
        } else {
            next_frame = now;
        }
    }
}
