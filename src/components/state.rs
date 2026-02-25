use wasm_bindgen::prelude::*;
use crate::components::creature::{Creature, Morphology};
use crate::components::policy::AttentionModel;
use crate::components::soft_body::SoftBodySimulation;

#[wasm_bindgen]
pub struct GameState {
    // Creature 1 (top lane, facing right)
    pub creature1_x: f32,
    pub creature1_y: f32,
    pub creature1_z: f32,
    pub creature1_rotation_y: f32,
    pub creature1_speed: f32,
    
    // Creature 2 (bottom lane, facing right) 
    pub creature2_x: f32,
    pub creature2_y: f32,
    pub creature2_z: f32,
    pub creature2_rotation_y: f32,
    pub creature2_speed: f32,
    
    // Racing game state
    pub game_started: bool,
    pub game_completed: bool,
    pub start_time: f64,
    pub current_time: f64,
    pub winner: i32,  // 0 = no winner, 1 = creature1 wins, 2 = creature2 wins
    
    #[allow(dead_code)]
    pub(crate) creature1: Creature,
    pub(crate) creature2: Creature,
    pub(crate) policy: Option<AttentionModel>,
    pub(crate) sim: Option<SoftBodySimulation>,
}

impl GameState {
    pub fn new(morphology: Morphology) -> Self {
        Self {
            // Creature 1: left side, facing right (0 degrees)
            creature1_x: -5.0,
            creature1_y: 1.0,
            creature1_z: 0.0,
            creature1_rotation_y: 0.0,
            creature1_speed: 0.0,
            
            // Creature 2: right side, facing left (180 degrees)
            creature2_x: 5.0,
            creature2_y: 1.0,
            creature2_z: 0.0,
            creature2_rotation_y: std::f32::consts::PI,
            creature2_speed: 0.0,
            
            // Tug-of-war game state
            rope_position: 0.0,
            game_started: false,
            game_completed: false,
            start_time: 0.0,
            current_time: 0.0,
            winner: 0,
            
            creature1: Creature::new(morphology),
            creature2: Creature::new(morphology),
            policy: None,
            sim: None,
        }
    }
}
