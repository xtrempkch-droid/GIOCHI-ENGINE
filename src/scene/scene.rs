//! Interface mínima para cenas atualizáveis e renderizáveis.
//! Status: trait de ciclo de atualização e desenho implementada.

use crate::platform::window::RenderContext;

/// Comportamento comum a menu, jogo e telas de pausa.
pub trait Scene {
    /// Atualiza a cena em segundos.
    fn update(&mut self, dt: f32);
    /// Desenha a cena atual.
    fn render(&mut self, ctx: &mut RenderContext<'_>);
}
