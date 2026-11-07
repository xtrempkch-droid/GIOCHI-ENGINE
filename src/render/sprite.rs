//! Sprite texturizado com recorte opcional da imagem de origem.
//! Status: desenho da textura com destino definido implementado.

use std::rc::Rc;

use crate::math::rect::Rect;

use super::{renderer::RenderContext, texture::Texture, RenderError};

/// Sprite que compartilha uma textura e pode desenhar uma região dela.
pub struct Sprite<'a> {
    /// Textura compartilhada pelo sprite.
    pub texture: Rc<Texture<'a>>,
    /// Região da textura usada pelo sprite; `None` seleciona a imagem inteira.
    pub src_rect: Option<Rect>,
}

impl<'a> Sprite<'a> {
    /// Cria um sprite usando toda a imagem como origem.
    pub fn new(texture: Rc<Texture<'a>>) -> Self {
        Self {
            texture,
            src_rect: None,
        }
    }

    /// Desenha o sprite no retângulo de destino informado.
    pub fn draw(
        &self,
        context: &mut RenderContext<'_>,
        destination: Rect,
    ) -> Result<(), RenderError> {
        context.draw_texture_ex(&self.texture, self.src_rect, destination, false, false, 0.0)
    }
}
