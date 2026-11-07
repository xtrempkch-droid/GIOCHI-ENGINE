//! Ciclo de vida, temporização e fachada de alto nível da engine.
//! Status: loop fixo, medição de tempo e `Engine` com cenas implementados.

pub mod engine;
pub mod game_loop;
pub mod time;

pub use engine::{bootstrap, Engine, EngineConfig, EngineError};
pub use game_loop::{FixedStepper, Game, FIXED_DT, MAX_FRAME_TIME};
pub use time::Time;
