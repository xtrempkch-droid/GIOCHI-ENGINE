//! Adaptadores de janela e desenho baseados no canvas SDL2.
//! Status: operações básicas de limpeza, retângulo e apresentação implementadas.

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window as SdlWindow};

use super::{sdl::Sdl, PlatformError};

/// Contexto de desenho simplificado para jogos.
pub struct RenderContext<'a> {
    canvas: &'a mut Canvas<SdlWindow>,
}

impl<'a> RenderContext<'a> {
    /// Empacota um canvas SDL2 existente.
    pub fn new(canvas: &'a mut Canvas<SdlWindow>) -> Self {
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

/// Janela SDL2 e canvas associado, com as dimensões solicitadas.
pub struct Window {
    canvas: Canvas<SdlWindow>,
    /// Largura inicial da janela em pixels.
    pub width: u32,
    /// Altura inicial da janela em pixels.
    pub height: u32,
}

impl Window {
    /// Cria uma janela SDL2 centralizada e seu canvas de desenho.
    pub fn new(sdl: &Sdl, title: &str, width: u32, height: u32) -> Result<Self, PlatformError> {
        let window = sdl
            .video
            .window(title, width, height)
            .position_centered()
            .allow_highdpi()
            .build()
            .map_err(|error| PlatformError::Window(error.to_string()))?;
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|error| PlatformError::Canvas(error.to_string()))?;

        Ok(Self {
            canvas,
            width,
            height,
        })
    }

    /// Limpa a janela com a cor indicada.
    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    /// Apresenta o conteúdo desenhado no canvas.
    pub fn present(&mut self) {
        self.canvas.present();
    }

    /// Cria um contexto de desenho emprestando temporariamente o canvas.
    pub fn render_context(&mut self) -> RenderContext<'_> {
        RenderContext::new(&mut self.canvas)
    }
}
