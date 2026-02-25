mod components;

use wasm_bindgen::prelude::*;
pub use crate::components::state::GameState;
use crate::components::creature::{Morphology, Creature};
use crate::components::policy::AttentionModel;
use crate::components::soft_body::SoftBodySimulation;

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new_wasm(morphology: u8) -> GameState {
        // This creates a new GameState for racing with two creatures
        let mut state = GameState {
            // Creature 1: top lane, facing right (0 degrees)
            creature1_x: -10.0,
            creature1_y: 1.0,
            creature1_z: 2.0,  // Top lane
            creature1_rotation_y: 0.0,
            creature1_speed: 0.0,
            
            // Creature 2: bottom lane, facing right (0 degrees)
            creature2_x: -10.0,
            creature2_y: 1.0,
            creature2_z: -2.0,  // Bottom lane
            creature2_rotation_y: 0.0,
            creature2_speed: 0.0,
            
            // Racing game state
            game_started: false,
            game_completed: false,
            start_time: 0.0,
            current_time: 0.0,
            winner: 0,
            
            creature1: Creature::new(match morphology {
                0 => Morphology::Biped,
                1 => Morphology::Quadruped,
                2 => Morphology::Hexapod,
                _ => Morphology::Biped,
            }),
            creature2: Creature::new(match morphology {
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
    pub fn start_game(&mut self, now: f64) {
        self.game_started = true;
        self.start_time = now;

        self.winner = 0;
    }

    #[wasm_bindgen]
    pub fn update(&mut self, delta: f32, now: f64, keys_json: &str) {
        if !self.game_started || self.game_completed {
            return;
        }

        self.current_time = (now - self.start_time) / 1000.0;

        // Parse JSON keys
        let keys: Vec<String> = match serde_json::from_str(keys_json) {
            Ok(keys) => keys,
            Err(_) => return,
        };

        // Racing physics constants
        let acceleration = 15.0;
        let max_speed = 30.0;
        let friction = 0.95;
        let finish_line_x = 20.0;

        // Player 1 controls (WASD)
        let p1_accelerate = keys.iter().any(|k| k == "KeyW");
        let p1_brake = keys.iter().any(|k| k == "KeyS");
        
        // Player 2 controls (Arrow keys)
        let p2_accelerate = keys.iter().any(|k| k == "ArrowUp");
        let p2_brake = keys.iter().any(|k| k == "ArrowDown");

        // Update creature 1 speed
        if p1_accelerate {
            self.creature1_speed += acceleration * delta;
        } else if p1_brake {
            self.creature1_speed -= acceleration * delta * 2.0;
        }
        self.creature1_speed = self.creature1_speed.clamp(0.0, max_speed);
        self.creature1_speed *= friction;

        // Update creature 2 speed
        if p2_accelerate {
            self.creature2_speed += acceleration * delta;
        } else if p2_brake {
            self.creature2_speed -= acceleration * delta * 2.0;
        }
        self.creature2_speed = self.creature2_speed.clamp(0.0, max_speed);
        self.creature2_speed *= friction;

        // Update positions
        self.creature1_x += self.creature1_speed * delta;
        self.creature2_x += self.creature2_speed * delta;

        // Check win conditions
        if !self.game_completed {
            if self.creature1_x > finish_line_x && self.creature2_x > finish_line_x {
                self.game_completed = true;
                self.winner = if self.creature1_x > self.creature2_x { 1 } else { 2 };
            } else if self.creature1_x > finish_line_x {
                self.game_completed = true;
                self.winner = 1;
            } else if self.creature2_x > finish_line_x {
                self.game_completed = true;
                self.winner = 2;
            }
        }
    }

    #[wasm_bindgen]
    pub fn get_creature1_x(&self) -> f32 { self.creature1_x }
    #[wasm_bindgen]
    pub fn get_creature1_y(&self) -> f32 { self.creature1_y }
    #[wasm_bindgen]
    pub fn get_creature1_z(&self) -> f32 { self.creature1_z }
    #[wasm_bindgen]
    pub fn get_creature1_rotation_y(&self) -> f32 { self.creature1_rotation_y }
    #[wasm_bindgen]
    pub fn get_creature1_speed(&self) -> f32 { self.creature1_speed }
    
    #[wasm_bindgen]
    pub fn get_creature2_x(&self) -> f32 { self.creature2_x }
    #[wasm_bindgen]
    pub fn get_creature2_y(&self) -> f32 { self.creature2_y }
    #[wasm_bindgen]
    pub fn get_creature2_z(&self) -> f32 { self.creature2_z }
    #[wasm_bindgen]
    pub fn get_creature2_rotation_y(&self) -> f32 { self.creature2_rotation_y }
    #[wasm_bindgen]
    pub fn get_creature2_speed(&self) -> f32 { self.creature2_speed }
    

    #[wasm_bindgen]
    pub fn is_completed(&self) -> bool { self.game_completed }
    #[wasm_bindgen]
    pub fn get_winner(&self) -> i32 { self.winner }
    #[wasm_bindgen]
    pub fn get_current_time(&self) -> f64 { self.current_time }


}
