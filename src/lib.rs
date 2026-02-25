mod components;

use wasm_bindgen::prelude::*;
pub use crate::components::state::GameState;
use crate::components::creature::{Morphology, Creature};
use crate::components::policy::AttentionModel;
use crate::components::soft_body::SoftBodySimulation;

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm(total_laps: i32, morphology: u8) -> GameState {
        // This creates a new GameState using the implementation in state.rs
        let mut state = GameState {
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
            creature: Creature::new(match morphology {
                0 => Morphology::Biped,
                1 => Morphology::Quadruped,
                2 => Morphology::Hexapod,
                _ => Morphology::Biped,
            }),
            policy: None,
            sim: None,
        };
        state
    }

    #[wasm_bindgen]
    pub fn load_policy(&mut self, args_json: &str, weights_json: &str) {
        let model = AttentionModel::new(args_json, weights_json);
        self.policy = Some(model);
    }

    #[wasm_bindgen]
    pub fn init_simulation(&mut self, mesh_json: &str) {
        let sim = SoftBodySimulation::new(mesh_json);
        self.sim = Some(sim);
    }

    #[wasm_bindgen]
    pub fn get_sim_node_positions(&self) -> JsValue {
        if let Some(sim) = &self.sim {
            let pos = sim.get_node_positions();
            serde_wasm_bindgen::to_value(&pos).unwrap()
        } else {
            JsValue::NULL
        }
    }

    #[wasm_bindgen]
    pub fn start_race(&mut self, now: f64) {
        self.race_started = true;
        self.start_time = now;
        self.last_angle = f32::atan2(self.z, self.x);
        self.total_angle = 0.0;
    }

    #[wasm_bindgen]
    pub fn update(&mut self, delta: f32, now: f64, keys_json: &str) {
        if !self.race_started || self.race_completed {
            return;
        }

        self.current_time = (now - self.start_time) / 1000.0;

        let base_speed = 15.0;
        let rotation_speed = 3.0;

        // Parse JSON keys
        let keys: Vec<String> = match serde_json::from_str(keys_json) {
            Ok(keys) => keys,
            Err(_) => return,
        };

        let move_forward = keys.iter().any(|k| k == "ArrowUp" || k == "KeyW");
        let move_backward = keys.iter().any(|k| k == "ArrowDown" || k == "KeyS" || k == "Space");
        let turn_left = keys.iter().any(|k| k == "ArrowLeft" || k == "KeyA");
        let turn_right = keys.iter().any(|k| k == "ArrowRight" || k == "KeyD");

        if move_forward {
            self.speed = base_speed;
        } else if move_backward {
            self.speed = -base_speed * 0.5;
        } else {
            self.speed *= 0.95;
        }

        if turn_left {
            self.rotation_y += delta * rotation_speed;
        }
        if turn_right {
            self.rotation_y -= delta * rotation_speed;
        }

        let distance = delta * self.speed;
        self.x += f32::sin(self.rotation_y) * distance;
        self.z += f32::cos(self.rotation_y) * distance;

        // Lap counting
        let current_angle = f32::atan2(self.z, self.x);
        let mut angle_diff = current_angle - self.last_angle;

        if angle_diff > std::f32::consts::PI {
            angle_diff -= std::f32::consts::PI * 2.0;
        }
        if angle_diff < -std::f32::consts::PI {
            angle_diff += std::f32::consts::PI * 2.0;
        }

        self.total_angle += angle_diff;
        self.last_angle = current_angle;

        let laps_completed = (self.total_angle.abs() / (std::f32::consts::PI * 2.0)) as i32;
        if laps_completed > self.lap {
            self.lap = laps_completed;
        }

        if self.lap >= self.total_laps {
            self.race_completed = true;
        }
    }

    #[wasm_bindgen]
    pub fn get_x(&self) -> f32 { self.x }
    #[wasm_bindgen]
    pub fn get_y(&self) -> f32 { self.y }
    #[wasm_bindgen]
    pub fn get_z(&self) -> f32 { self.z }
    #[wasm_bindgen]
    pub fn get_rotation_y(&self) -> f32 { self.rotation_y }
    #[wasm_bindgen]
    pub fn get_speed(&self) -> f32 { self.speed }
    #[wasm_bindgen]
    pub fn get_lap(&self) -> i32 { self.lap }
    #[wasm_bindgen]
    pub fn is_completed(&self) -> bool { self.race_completed }
    #[wasm_bindgen]
    pub fn get_current_time(&self) -> f64 { self.current_time }


}
