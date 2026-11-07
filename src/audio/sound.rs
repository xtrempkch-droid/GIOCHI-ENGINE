//! Efeitos sonoros e músicas carregados na memória.
//! Status: carregamento por arquivo, síntese de amostras, reprodução e controle implementados.

use std::{path::Path, rc::Rc};

use sdl2::mixer::{Channel, Chunk, Music as SdlMusic};

use super::AudioError;

/// Volume máximo aceito pelo SDL_mixer.
const MAX_VOLUME: i32 = 128;

/// Efeito sonoro curto, reutilizável em vários canais.
pub struct Sound {
    chunk: Chunk,
    volume: f32,
    source: String,
}

impl Sound {
    /// Carrega um efeito de um arquivo (WAV, OGG, FLAC conforme a build do mixer).
    ///
    /// O dispositivo de áudio precisa estar aberto, pois o SDL_mixer converte o
    /// recurso para o formato negociado com o backend.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, AudioError> {
        let path = path.as_ref();
        let chunk = Chunk::from_file(path).map_err(|message| AudioError::Load {
            path: path.display().to_string(),
            message,
        })?;
        Ok(Self {
            chunk,
            volume: 1.0,
            source: path.display().to_string(),
        })
    }

    /// Cria um efeito a partir de amostras PCM de 16 bits no formato do dispositivo.
    ///
    /// Útil para sons gerados em tempo de execução, sem arquivos externos.
    pub fn from_samples(samples: &[i16]) -> Result<Self, AudioError> {
        if samples.is_empty() {
            return Err(AudioError::Load {
                path: "<amostras>".to_owned(),
                message: "a lista de amostras está vazia".to_owned(),
            });
        }
        let chunk =
            Chunk::from_raw_buffer(samples.to_vec().into_boxed_slice()).map_err(|message| {
                AudioError::Load {
                    path: "<amostras>".to_owned(),
                    message,
                }
            })?;
        Ok(Self {
            chunk,
            volume: 1.0,
            source: "<amostras>".to_owned(),
        })
    }

    /// Gera uma onda senoidal simples, útil para testes e efeitos de interface.
    pub fn tone(
        frequency: f32,
        duration_seconds: f32,
        sample_rate: i32,
    ) -> Result<Self, AudioError> {
        let sample_rate = sample_rate.max(1);
        let total = (sample_rate as f32 * duration_seconds.max(0.0)) as usize;
        let samples: Vec<i16> = (0..total)
            .map(|index| {
                let phase = std::f32::consts::TAU * frequency * index as f32 / sample_rate as f32;
                (phase.sin() * f32::from(i16::MAX) * 0.25) as i16
            })
            .collect();
        Self::from_samples(&samples)
    }

    /// Reproduz uma vez em qualquer canal livre.
    pub fn play(&self) -> Result<(), AudioError> {
        self.play_loops(0)
    }

    /// Reproduz repetindo `loops` vezes; `-1` repete indefinidamente.
    pub fn play_loops(&self, loops: i32) -> Result<(), AudioError> {
        Channel::all()
            .play(&self.chunk, loops)
            .map(|_| ())
            .map_err(AudioError::Playback)
    }

    /// Reproduz em um canal específico.
    pub fn play_on(&self, channel: Channel, loops: i32) -> Result<Channel, AudioError> {
        channel
            .play(&self.chunk, loops)
            .map_err(AudioError::Playback)
    }

    /// Volume atual entre 0.0 e 1.0.
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Ajusta o volume do efeito, limitando o valor ao intervalo válido.
    pub fn set_volume(&mut self, volume: f32) -> f32 {
        self.volume = volume.clamp(0.0, 1.0);
        self.chunk.set_volume(to_mixer_volume(self.volume));
        self.volume
    }

    /// Origem do recurso, para mensagens de diagnóstico.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Compartilha o efeito sem recarregar o arquivo.
    pub fn into_shared(self) -> Rc<Self> {
        Rc::new(self)
    }
}

/// Música longa transmitida pelo mixer, com controle de reprodução.
pub struct MusicTrack {
    music: SdlMusic<'static>,
    volume: f32,
    source: String,
}

impl MusicTrack {
    /// Carrega uma música de arquivo.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, AudioError> {
        let path = path.as_ref();
        let music = SdlMusic::from_file(path).map_err(|message| AudioError::Load {
            path: path.display().to_string(),
            message,
        })?;
        Ok(Self {
            music,
            volume: 1.0,
            source: path.display().to_string(),
        })
    }

    /// Inicia a música uma vez.
    pub fn play(&self) -> Result<(), AudioError> {
        self.music.play(0).map_err(AudioError::Playback)
    }

    /// Inicia a música em repetição infinita.
    pub fn play_loop(&self) -> Result<(), AudioError> {
        self.music.play(-1).map_err(AudioError::Playback)
    }

    /// Repete a música `loops` vezes; `-1` repete indefinidamente.
    pub fn play_loops(&self, loops: i32) -> Result<(), AudioError> {
        self.music.play(loops).map_err(AudioError::Playback)
    }

    /// Pausa a música em reprodução.
    pub fn pause(&self) {
        SdlMusic::pause();
    }

    /// Retoma a música pausada.
    pub fn resume(&self) {
        SdlMusic::resume();
    }

    /// Interrompe a música imediatamente.
    pub fn stop(&self) {
        SdlMusic::halt();
    }

    /// Informa se há música tocando.
    pub fn is_playing(&self) -> bool {
        SdlMusic::is_playing()
    }

    /// Volume atual entre 0.0 e 1.0.
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Ajusta o volume da música em reprodução.
    pub fn set_volume(&mut self, volume: f32) -> f32 {
        self.volume = volume.clamp(0.0, 1.0);
        SdlMusic::set_volume(to_mixer_volume(self.volume));
        self.volume
    }

    /// Origem do recurso, para mensagens de diagnóstico.
    pub fn source(&self) -> &str {
        &self.source
    }
}

fn to_mixer_volume(volume: f32) -> i32 {
    (volume.clamp(0.0, 1.0) * MAX_VOLUME as f32).round() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_is_clamped_and_converted_to_mixer_range() {
        assert_eq!(to_mixer_volume(-1.0), 0);
        assert_eq!(to_mixer_volume(0.5), 64);
        assert_eq!(to_mixer_volume(2.0), MAX_VOLUME);
    }

    #[test]
    fn empty_sample_buffer_is_rejected() {
        assert!(matches!(
            Sound::from_samples(&[]),
            Err(AudioError::Load { .. })
        ));
    }
}
