//! Fachada de alto nível para inicializar e conduzir um jogo.
//! Status: configuração e orquestração do loop implementadas; SDL2 fica a cargo da aplicação.
//!
//! A criação de janela é delegada à camada de plataforma para manter SDL2 fora deste módulo.

use crate::{
    core::game_loop::{self, Game},
    render::renderer::RenderContext,
};

/// Configuração básica usada ao criar uma instância da engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineConfig {
    /// Largura lógica inicial da janela.
    pub width: u32,
    /// Altura lógica inicial da janela.
    pub height: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
        }
    }
}

/// Orquestra a execução do jogo com uma configuração de janela definida pelo consumidor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GameEngine {
    config: EngineConfig,
}

impl GameEngine {
    /// Cria a fachada da engine; a inicialização SDL2 permanece sob controle da aplicação.
    pub fn new(config: EngineConfig) -> Self {
        Self { config }
    }

    /// Retorna a configuração associada à engine.
    pub fn config(&self) -> EngineConfig {
        self.config
    }

    /// Executa o loop fixo até o callback de eventos indicar o encerramento.
    pub fn run<G, F>(
        &mut self,
        game: &mut G,
        context: &mut RenderContext<'_>,
        keep_running: F,
    ) -> anyhow::Result<()>
    where
        G: Game,
        F: FnMut(&mut G) -> bool,
    {
        game_loop::run_with_context(game, context, keep_running)
    }
}
