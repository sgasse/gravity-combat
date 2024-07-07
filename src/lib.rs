mod physics;
mod render;
mod utils;

use glam::Vec2;
use log::info;
use render::{get_window_dimensions, handle_input, set_canvas_size, Dimensions};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

const CANVAS_NAME: &str = "game";

#[wasm_bindgen]
pub fn wasm_main() {
    set_panic_hook();

    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    info!("Logging initialized");

    let Dimensions { width, height } = get_window_dimensions();
    set_canvas_size(CANVAS_NAME, width as u32, height as u32);

    handle_input();
}

struct Planet {
    position: Vec2,
    radius: f64,
}

struct Ship {
    position: Vec2,
    x_size: f64,
    y_size: f64,
    color: JsValue,
    angle: f64,
}

#[derive(Debug)]
enum MissileResult {
    OutOfRange,
    OutofFuel,
    HitPlanet,
    HitEnemy,
}
