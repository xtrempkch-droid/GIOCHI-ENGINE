//! Ciclo de vida do dispositivo de áudio SDL2_mixer.
//! Status: abertura, canais, consulta de especificação e encerramento implementados.

use sdl2::mixer::{self, InitFlag, Sdl2MixerContext, DEFAULT_CHANNELS, DEFAULT_FORMAT};

use super::AudioError;

/// Parâmetros para abrir o dispositivo de áudio SDL_mixer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MixerConfig {
    /// Frequência de amostragem em hertz.
    pub frequency: i32,
    /// Quantidade de canais de saída (1 mono, 2 estéreo).
    pub channels: i32,
    /// Tamanho do buffer em amostras.
    pub chunk_size: i32,
}

impl Default for MixerConfig {
    fn default() -> Self {
        Self {
            frequency: 44_100,
            channels: DEFAULT_CHANNELS,
            chunk_size: 1_024,
        }
    }
}

impl MixerConfig {
    /// Configuração mono, útil para efeitos simples e para síntese procedural.
    pub const MONO: Self = Self {
        frequency: 44_100,
        channels: 1,
        chunk_size: 1_024,
    };
}

/// Especificação efetivamente aceita pelo dispositivo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioSpec {
    /// Frequência obtida após a negociação com o backend.
    pub frequency: i32,
    /// Quantidade de canais obtida.
    pub channels: i32,
}

/// Dono do dispositivo de áudio; fechá-lo libera os canais e o mixer.
pub struct AudioSystem {
    config: MixerConfig,
    spec: AudioSpec,
    allocated_channels: i32,
    // Mantém o SDL2_mixer inicializado enquanto o dispositivo existir.
    _context: Sdl2MixerContext,
}

impl AudioSystem {
    /// Abre o dispositivo com a configuração indicada.
    pub fn init(config: MixerConfig) -> Result<Self, AudioError> {
        let context = mixer::init(InitFlag::empty()).map_err(AudioError::Initialization)?;
        mixer::open_audio(
            config.frequency,
            DEFAULT_FORMAT,
            config.channels,
            config.chunk_size,
        )
        .map_err(AudioError::DeviceOpen)?;
        let (frequency, _, channels) = mixer::query_spec().map_err(AudioError::DeviceOpen)?;

        Ok(Self {
            config,
            spec: AudioSpec {
                frequency,
                channels,
            },
            allocated_channels: 0,
            _context: context,
        })
    }

    /// Abre o dispositivo com a configuração padrão (44,1 kHz estéreo).
    pub fn with_defaults() -> Result<Self, AudioError> {
        Self::init(MixerConfig::default())
    }

    /// Configuração solicitada na abertura.
    pub fn config(&self) -> MixerConfig {
        self.config
    }

    /// Especificação aceita pelo dispositivo.
    pub fn spec(&self) -> AudioSpec {
        self.spec
    }

    /// Garante pelo menos `channels` canais de mixagem simultânea.
    ///
    /// Retorna a quantidade total de canais disponíveis. Valores menores ou
    /// iguais ao já alocado não reduzem os canais existentes.
    pub fn allocate_channels(&mut self, channels: i32) -> i32 {
        let requested = channels.max(1);
        if requested > self.allocated_channels {
            self.allocated_channels = mixer::allocate_channels(requested);
        }
        self.allocated_channels
    }

    /// Quantidade de canais de mixagem alocados pela engine.
    pub fn channels(&self) -> i32 {
        self.allocated_channels
    }

    /// Canais que estão tocando neste instante.
    pub fn playing_channels(&self) -> i32 {
        mixer::get_playing_channels_number()
    }
}

impl Drop for AudioSystem {
    fn drop(&mut self) {
        mixer::close_audio();
    }
}
