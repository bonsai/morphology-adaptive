mod components;

use wasm_bindgen::prelude::*;
pub use crate::components::state::GameState;
use crate::components::physics::PhysicsEngine;
use crate::components::rules::RaceRules;
use crate::components::creature::Morphology;

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm(total_laps: i32, morphology: u8) -> GameState {
        let morph = match morphology {
            0 => Morphology::Biped,
            1 => Morphology::Quadruped,
            2 => Morphology::Hexapod,
            _ => Morphology::Biped,
        };
        GameState::new(total_laps, morph)
    }

    pub fn start_race(&mut self, now: f64) {
        RaceRules::start(self, now);
    }

    pub fn update(&mut self, delta: f32, now: f64, keys: Vec<String>) {
        PhysicsEngine::update(self, delta, &keys);
        RaceRules::update(self, now);
    }

    pub fn get_x(&self) -> f32 { self.x }
    pub fn get_y(&self) -> f32 { self.y }
    pub fn get_z(&self) -> f32 { self.z }
    pub fn get_rotation_y(&self) -> f32 { self.rotation_y }
    pub fn get_speed(&self) -> f32 { self.speed }
    pub fn get_lap(&self) -> i32 { self.lap }
    pub fn is_completed(&self) -> bool { self.race_completed }
    pub fn get_current_time(&self) -> f64 { self.current_time }
}
