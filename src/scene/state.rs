//! Pilha simples para navegação entre estados da aplicação.
//! Status: operações básicas de pilha implementadas.

/// Gerencia a pilha de estados ativos.
#[derive(Debug, Default)]
pub struct StateStack<T> {
    states: Vec<T>,
}

impl<T> StateStack<T> {
    /// Empilha um estado e retorna o estado inserido.
    pub fn push(&mut self, state: T) -> &T {
        self.states.push(state);
        self.states.last().expect("o estado foi inserido")
    }

    /// Remove e retorna o estado do topo.
    pub fn pop(&mut self) -> Option<T> {
        self.states.pop()
    }

    /// Retorna o estado do topo sem removê-lo.
    pub fn current(&self) -> Option<&T> {
        self.states.last()
    }
}
