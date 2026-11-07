//! Gerenciador de cenas com pilha, substituição e transições pendentes.
//! Status: push, pop, replace, atualização e desenho do topo implementados.

use sdl2::event::Event;

use crate::{
    input::Input,
    render::{renderer::RenderContext, RenderError},
};

use super::{
    scene::{Scene, SceneContext},
    SceneTransition,
};

/// Gerencia a pilha de cenas ativas.
///
/// Transições pedidas durante `update` são aplicadas no início do quadro
/// seguinte, evitando que uma cena seja trocada enquanto ainda está sendo
/// executada. Isso mantém `push`/`pop`/`replace` seguros a partir de qualquer
/// método da cena.
pub struct SceneManager<'w> {
    stack: Vec<Box<dyn Scene + 'w>>,
    pending: Option<SceneTransition<'w>>,
}

impl<'w> SceneManager<'w> {
    /// Cria um gerenciador vazio.
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            pending: None,
        }
    }

    /// Cria o gerenciador com uma cena inicial, executando o `on_enter` dela.
    pub fn with_scene(scene: Box<dyn Scene + 'w>, ctx: &mut SceneContext<'_, 'w>) -> Self {
        let mut manager = Self::new();
        manager.push(scene);
        manager.apply_pending(ctx);
        manager
    }

    /// Agenda o empilhamento de uma cena; a cena atual continua ativa até o próximo quadro.
    pub fn push(&mut self, scene: Box<dyn Scene + 'w>) {
        if self.stack.is_empty() {
            // A primeira cena entra imediatamente para que `run` tenha o que desenhar.
            self.stack.push(scene);
            self.pending = Some(SceneTransition::Enter);
        } else {
            self.pending = Some(SceneTransition::Push(scene));
        }
    }

    /// Agenda a saída da cena do topo.
    pub fn pop(&mut self) {
        if !self.stack.is_empty() {
            self.pending = Some(SceneTransition::Pop);
        }
    }

    /// Agenda a substituição da cena do topo.
    pub fn replace(&mut self, scene: Box<dyn Scene + 'w>) {
        self.pending = Some(SceneTransition::Replace(scene));
    }

    /// Aplica a transição pendente, chamando `on_exit`/`on_enter` conforme necessário.
    pub fn apply_pending(&mut self, ctx: &mut SceneContext<'_, 'w>) {
        match self.pending.take() {
            None => {}
            Some(SceneTransition::Enter) => {
                if let Some(scene) = self.stack.last_mut() {
                    scene.on_enter(ctx);
                }
            }
            Some(SceneTransition::Push(scene)) => {
                if let Some(previous) = self.stack.last_mut() {
                    previous.on_exit();
                }
                self.stack.push(scene);
                if let Some(current) = self.stack.last_mut() {
                    current.on_enter(ctx);
                }
            }
            Some(SceneTransition::Pop) => {
                if let Some(mut removed) = self.stack.pop() {
                    removed.on_exit();
                }
                if let Some(previous) = self.stack.last_mut() {
                    previous.on_enter(ctx);
                }
            }
            Some(SceneTransition::Replace(scene)) => {
                if let Some(mut removed) = self.stack.pop() {
                    removed.on_exit();
                }
                self.stack.push(scene);
                if let Some(current) = self.stack.last_mut() {
                    current.on_enter(ctx);
                }
            }
        }
    }

    /// Atualiza apenas a cena do topo.
    pub fn update(&mut self, dt: f32, input: &Input) {
        if let Some(scene) = self.stack.last_mut() {
            scene.update(dt, input);
        }
    }

    /// Desenha apenas a cena do topo.
    pub fn render(&mut self, ctx: &mut RenderContext<'_>) -> Result<(), RenderError> {
        match self.stack.last_mut() {
            Some(scene) => scene.render(ctx),
            None => Ok(()),
        }
    }

    /// Encaminha um evento SDL para todas as cenas, do topo para a base.
    ///
    /// Retorna `true` quando alguma cena consumiu o evento.
    pub fn handle_event(&mut self, event: &Event) -> bool {
        self.stack
            .iter_mut()
            .rev()
            .any(|scene| scene.handle_event(event))
    }

    /// Cena do topo, se houver.
    pub fn current(&self) -> Option<&(dyn Scene + 'w)> {
        self.stack.last().map(|scene| scene.as_ref())
    }

    /// Cena do topo para mutação direta.
    pub fn current_mut(&mut self) -> Option<&mut (dyn Scene + 'w)> {
        self.stack.last_mut().map(|scene| scene.as_mut())
    }

    /// Quantidade de cenas na pilha.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Informa se a pilha está vazia.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl Default for SceneManager<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assets::ResourceManager, math::color::Color};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default)]
    struct Log {
        events: Rc<RefCell<Vec<String>>>,
    }

    struct TestScene {
        name: &'static str,
        log: Log,
        entered: usize,
        updated: usize,
    }

    impl TestScene {
        fn new(name: &'static str, log: &Log) -> Self {
            Self {
                name,
                log: Log {
                    events: Rc::clone(&log.events),
                },
                entered: 0,
                updated: 0,
            }
        }

        fn record(&self, action: &str) {
            self.log
                .events
                .borrow_mut()
                .push(format!("{}:{}", self.name, action));
        }
    }

    impl Scene for TestScene {
        fn on_enter(&mut self, _ctx: &mut SceneContext<'_, '_>) {
            self.entered += 1;
            self.record("enter");
        }

        fn on_exit(&mut self) {
            self.record("exit");
        }

        fn update(&mut self, _dt: f32, _input: &Input) {
            self.updated += 1;
            self.record("update");
        }

        fn render(&mut self, ctx: &mut RenderContext<'_>) -> Result<(), RenderError> {
            ctx.clear(Color::rgb(0, 0, 0));
            Ok(())
        }
    }

    fn context<'s, 'w>(resources: &'s mut ResourceManager<'w>) -> SceneContext<'s, 'w> {
        SceneContext {
            resources,
            audio: None,
        }
    }

    #[test]
    fn push_applies_on_enter_and_keeps_previous_scene_below() {
        let log = Log::default();
        let mut resources = ResourceManager::new();
        let mut manager = SceneManager::new();

        manager.push(Box::new(TestScene::new("menu", &log)));
        manager.apply_pending(&mut context(&mut resources));
        manager.push(Box::new(TestScene::new("game", &log)));
        manager.apply_pending(&mut context(&mut resources));

        assert_eq!(manager.depth(), 2);
        assert_eq!(
            log.events.borrow().as_slice(),
            ["menu:enter", "menu:exit", "game:enter"]
        );
    }

    #[test]
    fn pop_restores_previous_scene_and_calls_on_exit() {
        let log = Log::default();
        let mut resources = ResourceManager::new();
        let mut manager = SceneManager::new();
        manager.push(Box::new(TestScene::new("menu", &log)));
        manager.apply_pending(&mut context(&mut resources));
        manager.push(Box::new(TestScene::new("game", &log)));
        manager.apply_pending(&mut context(&mut resources));

        manager.pop();
        manager.apply_pending(&mut context(&mut resources));

        assert_eq!(manager.depth(), 1);
        assert_eq!(
            log.events.borrow().as_slice(),
            [
                "menu:enter",
                "menu:exit",
                "game:enter",
                "game:exit",
                "menu:enter"
            ]
        );
    }

    #[test]
    fn replace_swaps_only_the_top_scene() {
        let log = Log::default();
        let mut resources = ResourceManager::new();
        let mut manager = SceneManager::new();
        manager.push(Box::new(TestScene::new("menu", &log)));
        manager.apply_pending(&mut context(&mut resources));
        manager.push(Box::new(TestScene::new("game", &log)));
        manager.apply_pending(&mut context(&mut resources));

        manager.replace(Box::new(TestScene::new("pause", &log)));
        manager.apply_pending(&mut context(&mut resources));

        assert_eq!(manager.depth(), 2);
        assert_eq!(
            log.events.borrow().as_slice(),
            [
                "menu:enter",
                "menu:exit",
                "game:enter",
                "game:exit",
                "pause:enter"
            ]
        );
    }

    #[test]
    fn update_only_reaches_the_top_scene() {
        let log = Log::default();
        let mut resources = ResourceManager::new();
        let mut manager = SceneManager::new();
        manager.push(Box::new(TestScene::new("menu", &log)));
        manager.apply_pending(&mut context(&mut resources));
        manager.push(Box::new(TestScene::new("game", &log)));
        manager.apply_pending(&mut context(&mut resources));

        manager.update(0.016, &Input::headless());
        manager.update(0.016, &Input::headless());

        let events = log.events.borrow().clone();
        let updates: Vec<&str> = events
            .iter()
            .filter(|entry| entry.ends_with(":update"))
            .map(String::as_str)
            .collect();
        assert_eq!(updates, ["game:update", "game:update"]);
    }

    #[test]
    fn handle_event_is_consumed_from_top_to_bottom() {
        struct Consumer {
            consumed: bool,
        }

        impl Scene for Consumer {
            fn update(&mut self, _dt: f32, _input: &Input) {}

            fn render(&mut self, _ctx: &mut RenderContext<'_>) -> Result<(), RenderError> {
                Ok(())
            }

            fn handle_event(&mut self, event: &Event) -> bool {
                self.consumed = matches!(event, Event::KeyDown { .. });
                self.consumed
            }
        }

        let log = Log::default();
        let mut resources = ResourceManager::new();
        let mut manager = SceneManager::new();
        manager.push(Box::new(TestScene::new("base", &log)));
        manager.apply_pending(&mut context(&mut resources));
        manager.push(Box::new(Consumer { consumed: false }));
        manager.apply_pending(&mut context(&mut resources));

        let event = Event::KeyDown {
            timestamp: 0,
            window_id: 0,
            keycode: Some(sdl2::keyboard::Keycode::Space),
            scancode: None,
            keymod: sdl2::keyboard::Mod::NOMOD,
            repeat: false,
        };
        assert!(manager.handle_event(&event));
        assert!(!manager.handle_event(&Event::Quit { timestamp: 0 }));
    }
}
