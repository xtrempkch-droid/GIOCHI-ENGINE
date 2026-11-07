//! Configuração comum do mixer de áudio.
//! Status: configuração implementada; wrapper de mixer reutilizável ainda não implementado.

/// Parâmetros para abrir o dispositivo de áudio SDL_mixer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MixerConfig {
    /// Frequência de amostragem em hertz.
    pub frequency: i32,
    /// Quantidade de canais de saída.
    pub channels: i32,
    /// Tamanho do buffer em amostras.
    pub chunk_size: i32,
}

impl Default for MixerConfig {
    fn default() -> Self {
        Self {
            frequency: 44_100,
            channels: 2,
            chunk_size: 1_024,
        }
    }
}
