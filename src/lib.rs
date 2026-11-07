//! Engine 2D do GIOCHI-SYSTEM, organizada em módulos reutilizáveis.
//! Status: estrutura pública inicial implementada; alguns subsistemas são parciais.
//!
//! A infraestrutura de ciclo de vida, matemática e estados está implementada;
//! renderização SDL2 e plataforma fornecem a base funcional para a evolução da engine.

pub mod assets;
pub mod audio;
pub mod core;
pub mod input;
pub mod math;
pub mod platform;
pub mod render;
pub mod scene;
