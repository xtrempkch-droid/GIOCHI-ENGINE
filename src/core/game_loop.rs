//! Loop de jogo com passo fixo, acumulador e limite de tempo por quadro.
//! Status: entrada unificada, atualização fixa e encerramento por Quit/Escape implementados.

use std::time::{Duration, Instant};

use anyhow::Result;

use crate::{
    core::time::Time,
    input::Input,
    platform::{sdl::Sdl, window::Window},
    render::{renderer::RenderContext, RenderError},
};

/// Passo fixo de simulação, equivalente a 60 atualizações por segundo.
pub const FIXED_DT: f32 = 1.0 / 60.0;
/// Tempo máximo incorporado ao acumulador em um quadro.
pub const MAX_FRAME_TIME: f64 = 0.25;

/// Contrato mínimo implementado por um jogo controlado pela engine.
pub trait Game {
    /// Atualiza a simulação em segundos com o estado de entrada do quadro.
    ///
    /// A entrada reflete os eventos do quadro atual; transições `pressed` e
    /// `released` podem ser observadas por cada chamada de passo fixo.
    fn update(&mut self, dt: f32, input: &Input);
    /// Desenha um quadro e propaga falhas do backend de renderização.
    fn render(&mut self, ctx: &mut RenderContext<'_>) -> Result<(), RenderError>;
}

/// Executa o jogo até receber um evento de saída ou a tecla Escape.
pub fn run<G: Game>(game: &mut G, window: &mut Window, sdl: &mut Sdl) -> Result<()> {
    let mut input = Input::new(&sdl.ctx);
    run_fixed(
        game,
        &mut input,
        |input| sdl.poll_input(input),
        |game| {
            let mut context: RenderContext<'_> = window.render_context();
            game.render(&mut context)?;
            window.present();
            Ok(())
        },
    )?;
    Ok(())
}

/// Executa o mesmo loop fixo usando um contexto de desenho já emprestado.
///
/// Indicado para jogos que precisam manter texturas ou câmera entre quadros.
pub fn run_with_context<G: Game>(
    game: &mut G,
    ctx: &mut RenderContext<'_>,
    sdl: &mut Sdl,
) -> Result<()> {
    let mut input = Input::new(&sdl.ctx);
    run_fixed(
        game,
        &mut input,
        |input| sdl.poll_input(input),
        |game| {
            game.render(ctx)?;
            ctx.present();
            Ok(())
        },
    )?;
    Ok(())
}

fn run_fixed<G, P, R>(
    game: &mut G,
    input: &mut Input,
    mut poll_events: P,
    mut render: R,
) -> Result<(), RenderError>
where
    G: Game,
    P: FnMut(&mut Input) -> bool,
    R: FnMut(&mut G) -> Result<(), RenderError>,
{
    let fixed_dt = f64::from(FIXED_DT);
    let start = Instant::now();
    let mut time = Time::new(start);
    let mut accumulator = 0.0_f64;
    let mut next_frame = start;

    while poll_events(input) {
        accumulator += f64::from(time.tick(Instant::now()).min(MAX_FRAME_TIME as f32));

        while accumulator >= fixed_dt {
            game.update(FIXED_DT, input);
            accumulator -= fixed_dt;
        }

        render(game)?;

        next_frame += Duration::from_secs_f64(fixed_dt);
        let now = Instant::now();
        if next_frame > now {
            std::thread::sleep(next_frame - now);
        } else {
            next_frame = now;
        }
    }
    Ok(())
}
