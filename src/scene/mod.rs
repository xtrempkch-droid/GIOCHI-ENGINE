//! Cenas e navegação por pilha entre telas do jogo.
//! Status: contrato de cena, contexto de recursos e gerenciador de pilha implementados.

// A estrutura de arquivos segue o plano do projeto (`scene/scene.rs` para o trait),
// por isso o módulo tem o mesmo nome do pai.
#[allow(clippy::module_inception)]
pub mod scene;
pub mod state;

pub use scene::{Scene, SceneContext};
pub use state::SceneManager;

/// Transição de cena agendada para o próximo quadro.
pub enum SceneTransition<'w> {
    /// Executa `on_enter` da cena já empilhada.
    Enter,
    /// Empilha uma nova cena sobre a atual.
    Push(Box<dyn Scene + 'w>),
    /// Remove a cena do topo.
    Pop,
    /// Substitui a cena do topo.
    Replace(Box<dyn Scene + 'w>),
}
