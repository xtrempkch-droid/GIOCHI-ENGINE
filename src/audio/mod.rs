//! Áudio de efeitos e música sobre SDL2_mixer.
//! Status: dispositivo, canais, efeitos sonoros e música implementados.
//!
//! O jogo consome `AudioSystem`, `Sound` e `MusicTrack`; os tipos SDL_mixer ficam
//! encapsulados nesta camada para não vazar handles nativos.

pub mod mixer;
pub mod sound;

use thiserror::Error;

pub use mixer::{AudioSystem, MixerConfig};
pub use sound::{MusicTrack, Sound};

/// Falhas ao inicializar o áudio, carregar ou reproduzir recursos.
#[derive(Debug, Error)]
pub enum AudioError {
    /// O subsistema SDL2_mixer não pôde ser inicializado.
    #[error(
        "não foi possível inicializar o SDL2_mixer: {0}; verifique os pacotes de áudio da imagem"
    )]
    Initialization(String),
    /// O dispositivo de áudio não pôde ser aberto.
    #[error(
        "não foi possível abrir o dispositivo de áudio: {0}; verifique /dev/snd e o backend ALSA"
    )]
    DeviceOpen(String),
    /// O recurso de áudio não pôde ser carregado.
    #[error("não foi possível carregar o áudio '{path}': {message}")]
    Load {
        /// Caminho ou descrição do recurso.
        path: String,
        /// Mensagem devolvida pelo backend.
        message: String,
    },
    /// O backend recusou a reprodução ou o controle da música.
    #[error("falha na reprodução de áudio: {0}")]
    Playback(String),
}
