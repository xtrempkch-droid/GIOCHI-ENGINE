//! Carrega e mantém recursos de imagem SDL2 para desenho 2D.
//! Status: carregamento PNG por arquivo/bytes e criação RGBA procedural implementados.

use std::path::Path;

use sdl2::{
    image::{InitFlag, LoadTexture, Sdl2ImageContext},
    pixels::PixelFormatEnum,
    render::{Texture as SdlTexture, TextureCreator},
    video::WindowContext,
};

use super::RenderError;

/// Fábrica de texturas que mantém o suporte SDL_image ativo.
pub struct TextureLoader<'a> {
    creator: &'a TextureCreator<WindowContext>,
    _image_context: Sdl2ImageContext,
}

impl<'a> TextureLoader<'a> {
    pub(crate) fn new(creator: &'a TextureCreator<WindowContext>) -> Result<Self, RenderError> {
        let image_context = sdl2::image::init(InitFlag::PNG).map_err(RenderError::Image)?;
        Ok(Self {
            creator,
            _image_context: image_context,
        })
    }

    /// Carrega uma textura PNG a partir de um caminho.
    pub fn load(&self, path: impl AsRef<Path>) -> Result<Texture<'_>, RenderError> {
        Texture::from_file(self, path)
    }

    /// Carrega uma textura PNG a partir de bytes codificados.
    pub fn load_bytes(&self, bytes: &[u8]) -> Result<Texture<'_>, RenderError> {
        Texture::from_bytes(self, bytes)
    }

    /// Cria uma textura RGBA a partir de pixels no formato RGBA8.
    pub fn create_rgba(
        &self,
        width: u32,
        height: u32,
        pixels: &[u8],
    ) -> Result<Texture<'_>, RenderError> {
        if width == 0 || height == 0 {
            return Err(RenderError::InvalidImageDimensions);
        }
        let expected_len = width
            .checked_mul(height)
            .and_then(|pixels| pixels.checked_mul(4))
            .and_then(|bytes| usize::try_from(bytes).ok())
            .ok_or(RenderError::InvalidImageDimensions)?;
        if pixels.len() != expected_len {
            return Err(RenderError::InvalidImageData {
                expected: expected_len,
                actual: pixels.len(),
            });
        }

        let mut texture = self
            .creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, width, height)
            .map_err(|error| RenderError::Texture(error.to_string()))?;
        texture
            .with_lock(None, |target, pitch| {
                let row_bytes = width as usize * 4;
                for row in 0..height as usize {
                    let source_start = row * row_bytes;
                    let target_start = row * pitch;
                    target[target_start..target_start + row_bytes]
                        .copy_from_slice(&pixels[source_start..source_start + row_bytes]);
                }
            })
            .map_err(RenderError::Texture)?;

        Ok(Texture {
            inner: texture,
            width,
            height,
        })
    }
}

/// Textura carregada, emprestada da fábrica que a criou.
pub struct Texture<'a> {
    pub(crate) inner: SdlTexture<'a>,
    /// Largura da imagem em pixels.
    pub width: u32,
    /// Altura da imagem em pixels.
    pub height: u32,
}

impl<'a> Texture<'a> {
    /// Carrega um recurso de imagem usando a fábrica SDL_image.
    pub fn from_file(
        loader: &'a TextureLoader,
        path: impl AsRef<Path>,
    ) -> Result<Self, RenderError> {
        let inner = loader
            .creator
            .load_texture(path)
            .map_err(RenderError::Image)?;
        let query = inner.query();
        Ok(Self {
            inner,
            width: query.width,
            height: query.height,
        })
    }

    /// Decodifica bytes de imagem com SDL_image.
    pub fn from_bytes(loader: &'a TextureLoader, bytes: &[u8]) -> Result<Self, RenderError> {
        let inner = loader
            .creator
            .load_texture_bytes(bytes)
            .map_err(RenderError::Image)?;
        let query = inner.query();
        Ok(Self {
            inner,
            width: query.width,
            height: query.height,
        })
    }

    /// Cria uma textura RGBA procedural, útil para recursos gerados em memória.
    pub fn from_rgba(
        loader: &'a TextureLoader,
        width: u32,
        height: u32,
        pixels: &[u8],
    ) -> Result<Self, RenderError> {
        loader.create_rgba(width, height, pixels)
    }
}

impl Texture<'_> {
    /// Retorna uma visão dos metadados sem expor o objeto SDL2.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
