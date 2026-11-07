//! Adaptadores de janela e desenho baseados no canvas SDL2.
//! Status: operações básicas de limpeza, retângulo e apresentação implementadas.

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

/// Contexto de desenho simplificado para jogos.
pub struct RenderContext<'a> {
    canvas: &'a mut Canvas<Window>,
}

impl<'a> RenderContext<'a> {
    /// Empacota um canvas SDL2 existente.
    pub fn new(canvas: &'a mut Canvas<Window>) -> Self {
        Self { canvas }
    }

    /// Limpa a janela com a cor indicada.
    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    /// Desenha um retângulo preenchido.
    pub fn fill_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        self.canvas.set_draw_color(color);
        if let Err(error) = self.canvas.fill_rect(Rect::new(x, y, width, height)) {
            log::error!("falha ao desenhar retângulo: {error}");
        }
    }

    /// Apresenta o quadro renderizado.
    pub fn present(&mut self) {
        self.canvas.present();
    }
}
