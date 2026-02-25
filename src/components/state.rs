use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameState {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation_y: f32,
    pub speed: f32,
    pub lap: i32,
    pub total_laps: i32,
    pub last_angle: f32,
    pub total_angle: f32,
    pub race_started: bool,
    pub race_completed: bool,
    pub start_time: f64,
    pub current_time: f64,
}

impl GameState {
    pub fn new(total_laps: i32) -> Self {
        Self {
            x: 10.0,
            y: 1.0,
            z: 0.0,
            rotation_y: 0.0,
            speed: 0.0,
            lap: 0,
            total_laps,
            last_angle: 0.0,
            total_angle: 0.0,
            race_started: false,
            race_completed: false,
            start_time: 0.0,
            current_time: 0.0,
        }
    }
}
