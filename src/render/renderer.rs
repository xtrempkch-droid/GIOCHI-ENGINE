//! Contexto de desenho 2D independente das APIs de SDL2 expostas aos jogos.
//! Status: limpeza, primitivas, texturas e apresentação implementadas.

use sdl2::{
    pixels::PixelFormatEnum, rect::Rect as SdlRect, render::Canvas, video::Window as SdlWindow,
};

use crate::math::{color::Color, rect::Rect};

use super::{texture::Texture, RenderError};

/// Contexto de desenho que empresta temporariamente o canvas SDL2.
pub struct RenderContext<'a> {
    canvas: &'a mut Canvas<SdlWindow>,
}

impl<'a> RenderContext<'a> {
    /// Empacota um canvas SDL2 existente.
    pub(crate) fn new(canvas: &'a mut Canvas<SdlWindow>) -> Self {
        Self { canvas }
    }

    /// Limpa a área de desenho com a cor indicada.
    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(to_sdl_color(color));
        self.canvas.clear();
    }

    /// Desenha um retângulo preenchido.
    pub fn draw_rect(&mut self, rect: Rect, color: Color) -> Result<(), RenderError> {
        self.canvas.set_draw_color(to_sdl_color(color));
        self.canvas
            .fill_rect(to_sdl_rect(rect))
            .map_err(RenderError::Draw)
    }

    /// Desenha uma textura no destino indicado, sem rotação ou inversão.
    pub fn draw_texture(
        &mut self,
        texture: &Texture<'_>,
        destination: Rect,
    ) -> Result<(), RenderError> {
        self.draw_texture_ex(texture, None, destination, false, false, 0.0)
    }

    /// Desenha uma textura com recorte, inversão horizontal/vertical e rotação.
    pub fn draw_texture_ex(
        &mut self,
        texture: &Texture<'_>,
        source: Option<Rect>,
        destination: Rect,
        flip_horizontal: bool,
        flip_vertical: bool,
        angle_degrees: f64,
    ) -> Result<(), RenderError> {
        let source = source.map(to_sdl_rect);
        self.canvas
            .copy_ex(
                &texture.inner,
                source,
                to_sdl_rect(destination),
                angle_degrees,
                None,
                flip_horizontal,
                flip_vertical,
            )
            .map_err(RenderError::Draw)
    }

    /// Lê os pixels da área indicada no formato RGBA8888.
    ///
    /// Útil para capturas de tela e para testes de renderização headless que
    /// verificam a cor efetivamente produzida pelo backend.
    pub fn read_pixels(&self, rect: Rect) -> Result<Vec<u8>, RenderError> {
        self.canvas
            .read_pixels(to_sdl_rect(rect), PixelFormatEnum::RGBA32)
            .map_err(RenderError::Draw)
    }

    /// Apresenta o quadro renderizado.
    pub fn present(&mut self) {
        self.canvas.present();
    }
}

fn to_sdl_rect(rect: Rect) -> SdlRect {
    SdlRect::new(rect.x, rect.y, rect.width, rect.height)
}

fn to_sdl_color(color: Color) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGBA(color.red, color.green, color.blue, color.alpha)
}
