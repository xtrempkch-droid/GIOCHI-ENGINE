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

/// Executa um loop de passo fixo sem exigir a trait [`Game`].
///
/// Útil para fachadas que coordenam cenas: `poll` informa se o loop continua,
/// `update` roda uma vez por passo fixo e `render` desenha o quadro.
pub fn run_frames<P, U, R>(mut poll: P, mut update: U, mut render: R) -> Result<(), RenderError>
where
    P: FnMut() -> bool,
    U: FnMut(f32),
    R: FnMut() -> Result<(), RenderError>,
{
    let mut stepper = FixedStepper::new();
    while poll() {
        for _ in 0..stepper.advance() {
            update(FIXED_DT);
        }
        render()?;
        stepper.wait();
    }
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
    let mut stepper = FixedStepper::new();

    while poll_events(input) {
        for _ in 0..stepper.advance() {
            game.update(FIXED_DT, input);
        }
        render(game)?;
        stepper.wait();
    }
    Ok(())
}

/// Contabiliza tempo decorrido em passos fixos e limita a taxa de quadros.
///
/// É a peça reutilizável do loop: `advance` informa quantas atualizações de
/// [`FIXED_DT`] devem rodar no quadro e `wait` mantém a cadência de 60 Hz.
pub struct FixedStepper {
    time: Time,
    accumulator: f64,
    next_frame: Instant,
}

impl FixedStepper {
    /// Inicia a contagem a partir do instante atual.
    pub fn new() -> Self {
        let start = Instant::now();
        Self {
            time: Time::new(start),
            accumulator: 0.0,
            next_frame: start,
        }
    }

    /// Incorpora o tempo desde o último quadro e devolve quantos passos rodar.
    pub fn advance(&mut self) -> usize {
        self.accumulator += f64::from(self.time.tick(Instant::now()).min(MAX_FRAME_TIME as f32));
        let steps = (self.accumulator / f64::from(FIXED_DT)).floor() as usize;
        self.accumulator -= steps as f64 * f64::from(FIXED_DT);
        steps
    }

    /// Dorme até o próximo quadro para não consumir CPU além de 60 Hz.
    pub fn wait(&mut self) {
        self.next_frame += Duration::from_secs_f64(f64::from(FIXED_DT));
        let now = Instant::now();
        if self.next_frame > now {
            std::thread::sleep(self.next_frame - now);
        } else {
            self.next_frame = now;
        }
    }
}

impl Default for FixedStepper {
    fn default() -> Self {
        Self::new()
    }
}
