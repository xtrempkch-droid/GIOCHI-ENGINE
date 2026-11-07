//! Testes de integração que não exigem janela nem dispositivo de áudio.
//! Status: testes headless de cache de assets e colisão retangular.

use giochi_engine::{assets::manager::AssetManager, math::rect::Rect};

#[test]
fn asset_manager_reuses_inserted_values() {
    let mut manager = AssetManager::default();
    manager.insert("player", 7_u32);
    assert_eq!(manager.get("player"), Some(&7));
}

#[test]
fn rectangles_detect_intersection() {
    let first = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let second = Rect {
        x: 9,
        y: 9,
        width: 3,
        height: 3,
    };
    assert!(first.intersects(second));
}
