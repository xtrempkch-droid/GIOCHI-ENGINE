//! Loop de jogo com passo fixo, acumulador e limite de tempo por quadro.
//! Status: implementação funcional com callback de eventos.

use std::time::{Duration, Instant};

use anyhow::Result;

use crate::{
    core::time::Time,
    platform::{
        sdl::{Sdl, SdlEvent},
        window::{RenderContext, Window},
    },
};

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

/// Executa o jogo até receber um evento de saída ou a tecla Escape.
pub fn run<G: Game>(game: &mut G, window: &mut Window, sdl: &mut Sdl) -> Result<()> {
    run_fixed(
        game,
        |_| {
            let should_exit = sdl
                .poll_events()
                .iter()
                .any(|event| matches!(event, SdlEvent::Quit | SdlEvent::Escape));
            !should_exit
        },
        |game| {
            let mut context: RenderContext<'_> = window.render_context();
            game.render(&mut context);
            window.present();
        },
    );
    Ok(())
}

/// Executa o mesmo loop fixo com um contexto e uma rotina de eventos fornecidos pelo consumidor.
pub fn run_with_context<G, F>(game: &mut G, ctx: &mut RenderContext<'_>, keep_running: F)
where
    G: Game,
    F: FnMut(&mut G) -> bool,
{
    run_fixed(game, keep_running, |game| game.render(ctx));
}

fn run_fixed<G, P, R>(game: &mut G, mut poll_events: P, mut render: R)
where
    G: Game,
    P: FnMut(&mut G) -> bool,
    R: FnMut(&mut G),
{
    let fixed_dt = f64::from(FIXED_DT);
    let start = Instant::now();
    let mut time = Time::new(start);
    let mut accumulator = 0.0_f64;
    let mut next_frame = start;

    while poll_events(game) {
        accumulator += f64::from(time.tick(Instant::now()).min(MAX_FRAME_TIME as f32));

        while accumulator >= fixed_dt {
            game.update(FIXED_DT);
            accumulator -= fixed_dt;
        }

        render(game);

        next_frame += Duration::from_secs_f64(fixed_dt);
        let now = Instant::now();
        if next_frame > now {
            std::thread::sleep(next_frame - now);
        } else {
            next_frame = now;
        }
    }
}
