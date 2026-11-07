//! Janela SDL2 e adaptação ao contexto de renderização da engine.
//! Status: criação de janela, contexto, fábrica de texturas e desmembramento implementados.

use sdl2::{
    rect::Rect as SdlRect,
    render::{Canvas, TextureCreator},
    video::{Window as SdlWindow, WindowContext},
};

use crate::{
    math::{color::Color, rect::Rect},
    render::{texture::TextureLoader, RenderError},
};

use super::{sdl::Sdl, PlatformError};

pub use crate::render::renderer::RenderContext;

/// Alvo de desenho que empresta apenas o canvas, sem o criador de texturas.
///
/// Existe para permitir que `core::Engine` guarde simultaneamente o alvo mutável e
/// o cache de texturas (que empresta o criador imutavelmente): os dois vêm de
/// campos distintos de [`Window`], então o verificador de empréstimos aceita a
/// divisão sem `unsafe` nem auto-referência.
pub struct RenderTarget<'w> {
    canvas: &'w mut Canvas<SdlWindow>,
}

impl RenderTarget<'_> {
    /// Cria um contexto de desenho emprestando o canvas por um quadro.
    pub fn context(&mut self) -> RenderContext<'_> {
        RenderContext::new(self.canvas)
    }
}

/// Janela SDL2 e canvas associado, com as dimensões solicitadas.
pub struct Window {
    canvas: Canvas<SdlWindow>,
    texture_creator: TextureCreator<WindowContext>,
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
        let texture_creator = canvas.texture_creator();

        Ok(Self {
            canvas,
            texture_creator,
            width,
            height,
        })
    }

    /// Limpa a janela com a cor indicada.
    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(to_sdl_color(color));
        self.canvas.clear();
    }

    /// Desenha um retângulo preenchido e reporta erros do backend SDL2.
    pub fn draw_rect(&mut self, rect: Rect, color: Color) -> Result<(), RenderError> {
        self.canvas.set_draw_color(to_sdl_color(color));
        self.canvas
            .fill_rect(SdlRect::new(rect.x, rect.y, rect.width, rect.height))
            .map_err(RenderError::Draw)
    }

    /// Apresenta o conteúdo desenhado no canvas.
    pub fn present(&mut self) {
        self.canvas.present();
    }

    /// Divide a janela em alvo de desenho e criador de texturas independentes.
    ///
    /// Os dois empréstimos apontam para campos distintos e podem coexistir pelo
    /// tempo de vida `'w`, o que sustenta a fachada [`crate::core::Engine`].
    pub fn split<'w>(&'w mut self) -> (RenderTarget<'w>, &'w TextureCreator<WindowContext>) {
        let Self {
            canvas,
            texture_creator,
            ..
        } = self;
        (RenderTarget { canvas }, texture_creator)
    }

    /// Cria um contexto de desenho emprestando temporariamente o canvas.
    pub fn render_context(&mut self) -> RenderContext<'_> {
        RenderContext::new(&mut self.canvas)
    }

    /// Empresta simultaneamente o contexto de desenho e uma fábrica de texturas.
    ///
    /// As texturas criadas pela fábrica permanecem vinculadas ao empréstimo da janela,
    /// garantindo que não sobrevivam ao canvas SDL2 que as originou.
    pub fn render_context_with_loader(
        &mut self,
    ) -> Result<(RenderContext<'_>, TextureLoader<'_>), RenderError> {
        let Self {
            canvas,
            texture_creator,
            ..
        } = self;
        let loader = TextureLoader::new(texture_creator)?;
        let context = RenderContext::new(canvas);
        Ok((context, loader))
    }
}

fn to_sdl_color(color: Color) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGBA(color.red, color.green, color.blue, color.alpha)
}
