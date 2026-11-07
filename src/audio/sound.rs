//! Identificador simples de um som carregado.
//! Status: tipo de som implementado; carregamento e reprodução são parciais.

/// Recurso de áudio referenciado por caminho e volume.
#[derive(Debug, Clone, PartialEq)]
pub struct Sound {
    /// Caminho de origem do recurso.
    pub path: String,
    /// Volume entre 0.0 e 1.0.
    pub volume: f32,
}

impl Sound {
    /// Cria um som com volume máximo.
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            volume: 1.0,
        }
    }
}
