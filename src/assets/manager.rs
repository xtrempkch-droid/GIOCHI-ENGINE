//! Cache genérico de recursos indexado por identificador textual.
//! Status: cache genérico em memória implementado.

use std::collections::HashMap;

/// Armazena recursos já carregados para evitar leituras duplicadas.
#[derive(Debug, Default)]
pub struct AssetManager<T> {
    assets: HashMap<String, T>,
}

impl<T> AssetManager<T> {
    /// Insere ou substitui um recurso.
    pub fn insert(&mut self, key: impl Into<String>, asset: T) -> Option<T> {
        self.assets.insert(key.into(), asset)
    }

    /// Obtém uma referência ao recurso associado à chave.
    pub fn get(&self, key: &str) -> Option<&T> {
        self.assets.get(key)
    }

    /// Número de recursos armazenados.
    pub fn len(&self) -> usize {
        self.assets.len()
    }

    /// Informa se o cache não contém recursos.
    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }
}
