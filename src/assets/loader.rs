//! Funções de carregamento de recursos e validação de caminhos.
//! Status: leitura de bytes, textura, som, música e fonte implementados.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::{
    audio::{MusicTrack, Sound},
    render::{
        texture::{Texture, TextureLoader},
        RenderError,
    },
};

use super::AssetError;

/// Verifica se o caminho existe e aponta para um arquivo regular.
pub fn validate_asset_path(path: impl AsRef<Path>) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    if metadata.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "o caminho do recurso não é um arquivo",
        ))
    }
}

/// Lê o recurso inteiro, distinguindo caminho inválido de falha de leitura.
pub fn read_bytes(path: impl AsRef<Path>) -> Result<Vec<u8>, AssetError> {
    let path = path.as_ref();
    validate_asset_path(path).map_err(|error| match error.kind() {
        io::ErrorKind::NotFound | io::ErrorKind::InvalidInput => AssetError::InvalidPath {
            path: path.display().to_string(),
            message: error.to_string(),
        },
        _ => AssetError::Io {
            path: path.display().to_string(),
            message: error.to_string(),
        },
    })?;
    fs::read(path).map_err(|error| AssetError::Io {
        path: path.display().to_string(),
        message: error.to_string(),
    })
}

/// Carrega uma textura de imagem pelo caminho.
pub fn load_texture<'a>(
    loader: &'a TextureLoader<'_>,
    path: impl AsRef<Path>,
) -> Result<Texture<'a>, AssetError> {
    let path = path.as_ref();
    read_bytes(path)?;
    Texture::from_file(loader, path).map_err(|error| texture_error(path, error))
}

/// Carrega uma textura a partir de bytes já lidos do disco.
pub fn load_texture_bytes<'a>(
    loader: &'a TextureLoader<'_>,
    path: impl AsRef<Path>,
    bytes: &[u8],
) -> Result<Texture<'a>, AssetError> {
    let path = path.as_ref();
    Texture::from_bytes(loader, bytes).map_err(|error| texture_error(path, error))
}

/// Carrega um efeito sonoro curto.
pub fn load_sound(path: impl AsRef<Path>) -> Result<Sound, AssetError> {
    let path = path.as_ref();
    read_bytes(path)?;
    Sound::from_file(path).map_err(|error| AssetError::Audio {
        path: path.display().to_string(),
        message: error.to_string(),
    })
}

/// Carrega uma música transmitida pelo mixer.
pub fn load_music(path: impl AsRef<Path>) -> Result<MusicTrack, AssetError> {
    let path = path.as_ref();
    read_bytes(path)?;
    MusicTrack::from_file(path).map_err(|error| AssetError::Audio {
        path: path.display().to_string(),
        message: error.to_string(),
    })
}

fn texture_error(path: &Path, error: RenderError) -> AssetError {
    AssetError::Texture {
        path: path.display().to_string(),
        message: error.to_string(),
    }
}

/// Dono do SDL_ttf e fábrica de fontes por tamanho.
pub struct FontLoader {
    context: Sdl2TtfContext,
    /// Caminho base usado para resolver nomes relativos, quando definido.
    pub base_dir: Option<PathBuf>,
}

impl FontLoader {
    /// Inicializa o SDL_ttf.
    pub fn new() -> Result<Self, AssetError> {
        let context = sdl2::ttf::init().map_err(|message| AssetError::Font {
            path: "<SDL_ttf>".to_owned(),
            size: 0,
            message,
        })?;
        Ok(Self {
            context,
            base_dir: None,
        })
    }

    /// Inicializa o SDL_ttf com um diretório base para recursos relativos.
    pub fn with_base_dir(base_dir: impl Into<PathBuf>) -> Result<Self, AssetError> {
        let mut loader = Self::new()?;
        loader.base_dir = Some(base_dir.into());
        Ok(loader)
    }

    /// Carrega uma fonte no corpo indicado em pontos.
    pub fn load(
        &self,
        path: impl AsRef<Path>,
        point_size: u16,
    ) -> Result<Font<'_, 'static>, AssetError> {
        let path = self.resolve(path.as_ref());
        read_bytes(&path)?;
        self.context
            .load_font(&path, point_size)
            .map_err(|message| AssetError::Font {
                path: path.display().to_string(),
                message,
                size: point_size,
            })
    }

    /// Resolve um caminho relativo contra o diretório base, quando houver.
    pub fn resolve(&self, path: &Path) -> PathBuf {
        match &self.base_dir {
            Some(base) if path.is_relative() => base.join(path),
            _ => path.to_path_buf(),
        }
    }
}
