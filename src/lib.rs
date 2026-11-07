//! Engine 2D do GIOCHI-SYSTEM, organizada em módulos reutilizáveis.
//! Status: estrutura pública inicial implementada; alguns subsistemas são parciais.
//!
//! A fachada [`core::Engine`] reúne plataforma SDL2/KMSDRM, entrada, renderização,
//! cache de recursos, áudio e pilha de cenas. Camadas de nível mais baixo
//! (`core::game_loop::Game`, `platform::Window`) continuam públicas para jogos
//! simples e para testes headless.

pub mod assets;
pub mod audio;
pub mod core;
pub mod input;
pub mod math;
pub mod platform;
pub mod render;
pub mod scene;
