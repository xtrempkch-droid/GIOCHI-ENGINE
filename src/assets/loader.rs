//! Utilitários para validar caminhos de recursos antes da integração SDL_image.
//! Status: validação de arquivo implementada; decodificação SDL_image ainda não implementada.

use std::{io, path::Path};

/// Verifica se o caminho existe e aponta para um arquivo regular.
pub fn validate_asset_path(path: impl AsRef<Path>) -> io::Result<()> {
    let metadata = std::fs::metadata(path)?;
    if metadata.is_file() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "o caminho do recurso não é um arquivo",
        ))
    }
}
