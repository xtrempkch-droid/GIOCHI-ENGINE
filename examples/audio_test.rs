//! Gera e reproduz um tom de teste sem depender de um arquivo de áudio externo.
//! Status: teste funcional de saída SDL_mixer quando há dispositivo de áudio.

use std::{f64::consts::TAU, fs::File, io::Write, path::Path, thread, time::Duration};

use anyhow::{Context, Result};
use sdl2::mixer::{self, Channel, Chunk, DEFAULT_CHANNELS, DEFAULT_FORMAT};

fn main() -> Result<()> {
    let temporary_directory = tempfile::tempdir()?;
    let wave_path = temporary_directory.path().join("giochi-audio-test.wav");
    write_test_tone(&wave_path)?;

    mixer::open_audio(44_100, DEFAULT_FORMAT, DEFAULT_CHANNELS, 1_024)
        .map_err(anyhow::Error::msg)
        .context("não foi possível abrir o dispositivo SDL_mixer")?;
    mixer::allocate_channels(1);
    let chunk = Chunk::from_file(&wave_path).map_err(anyhow::Error::msg)?;
    Channel::all()
        .play(&chunk, 0)
        .map_err(anyhow::Error::msg)
        .context("não foi possível reproduzir o tom de teste")?;
    thread::sleep(Duration::from_millis(1_100));
    mixer::close_audio();
    Ok(())
}

fn write_test_tone(path: &Path) -> Result<()> {
    let sample_rate = 44_100_u32;
    let sample_count = sample_rate;
    let data_size = sample_count * 2;
    let mut file = File::create(path)?;
    file.write_all(b"RIFF")?;
    file.write_all(&(36 + data_size).to_le_bytes())?;
    file.write_all(b"WAVEfmt ")?;
    file.write_all(&16_u32.to_le_bytes())?;
    file.write_all(&1_u16.to_le_bytes())?;
    file.write_all(&1_u16.to_le_bytes())?;
    file.write_all(&sample_rate.to_le_bytes())?;
    file.write_all(&(sample_rate * 2).to_le_bytes())?;
    file.write_all(&2_u16.to_le_bytes())?;
    file.write_all(&16_u16.to_le_bytes())?;
    file.write_all(b"data")?;
    file.write_all(&data_size.to_le_bytes())?;

    for index in 0..sample_count {
        let phase = TAU * 440.0 * f64::from(index) / f64::from(sample_rate);
        let sample = (phase.sin() * f64::from(i16::MAX) * 0.2) as i16;
        file.write_all(&sample.to_le_bytes())?;
    }

    Ok(())
}
