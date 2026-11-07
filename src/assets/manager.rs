//! Caches de recursos carregados, com estatísticas de acerto e falta.
//! Status: cache genérico, ResourceManager e FontCache implementados.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
};

use sdl2::ttf::Font;

use crate::{
    audio::{MusicTrack, Sound},
    render::texture::{Texture, TextureLoader},
};

use super::{loader, AssetError, FontLoader};

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

/// Contadores de uso do cache.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CacheStats {
    hits: u64,
    misses: u64,
}

impl CacheStats {
    /// Requisições atendidas sem recarregar o arquivo.
    pub fn hits(&self) -> u64 {
        self.hits
    }

    /// Requisições que precisaram carregar o recurso.
    pub fn misses(&self) -> u64 {
        self.misses
    }

    /// Total de recursos efetivamente carregados do disco/backend.
    pub fn loads(&self) -> u64 {
        self.misses
    }

    /// Total de requisições ao cache.
    pub fn requests(&self) -> u64 {
        self.hits + self.misses
    }

    /// Zera os contadores.
    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}

/// Cache de recursos compartilhados por `Rc`.
///
/// As texturas emprestam o criador SDL da janela; o lifetime `'w` impede que o
/// cache sobreviva ao canvas que as originou. Fontes ficam em [`FontCache`]
/// porque dependem do `Sdl2TtfContext` de um `FontLoader` específico.
pub struct ResourceManager<'w> {
    textures: HashMap<PathBuf, Rc<Texture<'w>>>,
    sounds: HashMap<PathBuf, Rc<Sound>>,
    music: HashMap<PathBuf, Rc<MusicTrack>>,
    stats: CacheStats,
}

impl<'w> ResourceManager<'w> {
    /// Cria um cache vazio.
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            sounds: HashMap::new(),
            music: HashMap::new(),
            stats: CacheStats::default(),
        }
    }

    /// Carrega (ou reutiliza) uma textura pelo caminho.
    pub fn texture(
        &mut self,
        loader: &TextureLoader<'w>,
        path: impl AsRef<Path>,
    ) -> Result<Rc<Texture<'w>>, AssetError> {
        let path = path.as_ref();
        if let Some(texture) = self.textures.get(path) {
            self.stats.hits += 1;
            return Ok(Rc::clone(texture));
        }
        self.stats.misses += 1;
        let texture = Rc::new(loader::load_texture(loader, path)?);
        self.textures
            .insert(path.to_path_buf(), Rc::clone(&texture));
        Ok(texture)
    }

    /// Carrega (ou reutiliza) um efeito sonoro pelo caminho.
    pub fn sound(&mut self, path: impl AsRef<Path>) -> Result<Rc<Sound>, AssetError> {
        let path = path.as_ref();
        if let Some(sound) = self.sounds.get(path) {
            self.stats.hits += 1;
            return Ok(Rc::clone(sound));
        }
        self.stats.misses += 1;
        let sound = Rc::new(loader::load_sound(path)?);
        self.sounds.insert(path.to_path_buf(), Rc::clone(&sound));
        Ok(sound)
    }

    /// Carrega (ou reutiliza) uma música pelo caminho.
    pub fn music(&mut self, path: impl AsRef<Path>) -> Result<Rc<MusicTrack>, AssetError> {
        let path = path.as_ref();
        if let Some(music) = self.music.get(path) {
            self.stats.hits += 1;
            return Ok(Rc::clone(music));
        }
        self.stats.misses += 1;
        let music = Rc::new(loader::load_music(path)?);
        self.music.insert(path.to_path_buf(), Rc::clone(&music));
        Ok(music)
    }

    /// Quantidade de texturas em cache.
    pub fn texture_count(&self) -> usize {
        self.textures.len()
    }

    /// Quantidade de efeitos sonoros em cache.
    pub fn sound_count(&self) -> usize {
        self.sounds.len()
    }

    /// Quantidade de músicas em cache.
    pub fn music_count(&self) -> usize {
        self.music.len()
    }

    /// Quantidade total de recursos em cache.
    pub fn len(&self) -> usize {
        self.textures.len() + self.sounds.len() + self.music.len()
    }

    /// Informa se o cache não contém recursos.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Estatísticas acumuladas de uso.
    pub fn stats(&self) -> CacheStats {
        self.stats
    }

    /// Esvazia todos os caches e os contadores.
    pub fn clear(&mut self) {
        self.textures.clear();
        self.sounds.clear();
        self.music.clear();
        self.stats.reset();
    }
}

impl Default for ResourceManager<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache de fontes por `(caminho, corpo)`.
///
/// Separado do [`ResourceManager`] porque `Font<'t, 'static>` empresta o
/// `Sdl2TtfContext` do `FontLoader`, que tem lifetime próprio.
pub struct FontCache<'t> {
    fonts: HashMap<(PathBuf, u16), Rc<Font<'t, 'static>>>,
    stats: CacheStats,
}

impl<'t> FontCache<'t> {
    /// Cria um cache vazio.
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            stats: CacheStats::default(),
        }
    }

    /// Carrega (ou reutiliza) uma fonte pelo par caminho/corpo.
    pub fn font(
        &mut self,
        loader: &'t FontLoader,
        path: impl AsRef<Path>,
        point_size: u16,
    ) -> Result<Rc<Font<'t, 'static>>, AssetError> {
        let path = loader.resolve(path.as_ref());
        let key = (path.clone(), point_size);
        if let Some(font) = self.fonts.get(&key) {
            self.stats.hits += 1;
            return Ok(Rc::clone(font));
        }
        self.stats.misses += 1;
        let font = Rc::new(loader.load(&path, point_size)?);
        self.fonts.insert(key, Rc::clone(&font));
        Ok(font)
    }

    /// Quantidade de fontes em cache.
    pub fn len(&self) -> usize {
        self.fonts.len()
    }

    /// Informa se o cache não contém fontes.
    pub fn is_empty(&self) -> bool {
        self.fonts.is_empty()
    }

    /// Estatísticas acumuladas de uso.
    pub fn stats(&self) -> CacheStats {
        self.stats
    }

    /// Esvazia o cache e zera os contadores.
    pub fn clear(&mut self) {
        self.fonts.clear();
        self.stats.reset();
    }
}

impl Default for FontCache<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_stats_track_hits_misses_and_requests() {
        let mut stats = CacheStats::default();
        assert_eq!(
            (
                stats.hits(),
                stats.misses(),
                stats.requests(),
                stats.loads()
            ),
            (0, 0, 0, 0)
        );

        stats.hits += 3;
        stats.misses += 2;
        assert_eq!(stats.requests(), 5);
        assert_eq!(stats.loads(), 2, "loads é o número de cargas reais");

        stats.reset();
        assert_eq!(stats.requests(), 0);
    }

    #[test]
    fn generic_asset_manager_replaces_and_counts() {
        let mut manager = AssetManager::default();
        assert!(manager.is_empty());

        assert_eq!(manager.insert("a", 1), None);
        assert_eq!(
            manager.insert("a", 2),
            Some(1),
            "substituição devolve o antigo"
        );
        assert_eq!(manager.get("a"), Some(&2));
        assert_eq!(manager.get("b"), None);
        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn resource_manager_clear_empties_every_cache_and_resets_stats() {
        let mut resources = ResourceManager::new();
        resources.stats.hits += 4;
        resources.stats.misses += 2;

        resources.clear();

        assert!(resources.is_empty());
        assert_eq!(resources.texture_count(), 0);
        assert_eq!(resources.sound_count(), 0);
        assert_eq!(resources.music_count(), 0);
        assert_eq!(resources.stats().requests(), 0);
    }

    #[test]
    fn font_cache_starts_empty() {
        let mut fonts = FontCache::new();
        assert!(fonts.is_empty());
        assert_eq!(fonts.len(), 0);
        fonts.clear();
        assert_eq!(fonts.stats().requests(), 0);
    }
}
