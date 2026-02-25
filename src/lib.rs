use wasm_bindgen::prelude::*;
use nalgebra::{Vector3, Isometry3};
use std::collections::HashMap;

// Enable better panic messages in debug mode
#[cfg(feature = "console_error_panic_hook")]
use console_error_panic_hook;

// Use wee_alloc as the global allocator in production
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Morphology types supported in the game
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Morphology {
    Biped,
    Quadruped,
    Hexapod,
}

// Muscle structure for controlling limb movements
#[derive(Debug, Clone)]
pub struct Muscle {
    pub origin: Vector3<f32>,
    pub insertion: Vector3<f32>,
    pub max_force: f32,
    pub activation: f32,
}

// Limb structure
#[derive(Debug, Clone)]
pub struct Limb {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub length: f32,
    pub muscles: Vec<Muscle>,
}

// Creature structure with morphology-specific configuration
#[derive(Debug, Clone)]
pub struct Creature {
    pub morphology: Morphology,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub limbs: Vec<Limb>,
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    pub mass: f32,
}

impl Creature {
    pub fn new(morphology: Morphology) -> Self {
        let (limbs, mass) = match morphology {
            Morphology::Biped => (Self::create_biped_limbs(), 50.0),
            Morphology::Quadruped => (Self::create_quadruped_limbs(), 80.0),
            Morphology::Hexapod => (Self::create_hexapod_limbs(), 120.0),
        };

        Creature {
            morphology,
            position: Vector3::new(0.0, 1.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            limbs,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            acceleration: Vector3::new(0.0, 0.0, 0.0),
            mass,
        }
    }

    fn create_biped_limbs() -> Vec<Limb> {
        vec![
            // Left leg
            Limb {
                position: Vector3::new(-0.5, 0.0, 0.0),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 1.0,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -1.0, 0.0),
                        max_force: 800.0,
                        activation: 0.0,
                    },
                ],
            },
            // Right leg
            Limb {
                position: Vector3::new(0.5, 0.0, 0.0),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 1.0,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -1.0, 0.0),
                        max_force: 800.0,
                        activation: 0.0,
                    },
                ],
            },
        ]
    }

    fn create_quadruped_limbs() -> Vec<Limb> {
        vec![
            // Front left leg
            Limb {
                position: Vector3::new(-0.5, 0.0, 0.5),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 0.8,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -0.8, 0.0),
                        max_force: 600.0,
                        activation: 0.0,
                    },
                ],
            },
            // Front right leg
            Limb {
                position: Vector3::new(0.5, 0.0, 0.5),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 0.8,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -0.8, 0.0),
                        max_force: 600.0,
                        activation: 0.0,
                    },
                ],
            },
            // Back left leg
            Limb {
                position: Vector3::new(-0.5, 0.0, -0.5),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 0.8,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -0.8, 0.0),
                        max_force: 600.0,
                        activation: 0.0,
                    },
                ],
            },
            // Back right leg
            Limb {
                position: Vector3::new(0.5, 0.0, -0.5),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 0.8,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -0.8, 0.0),
                        max_force: 600.0,
                        activation: 0.0,
                    },
                ],
            },
        ]
    }

    fn create_hexapod_limbs() -> Vec<Limb> {
        vec![
            // Front left leg
            Limb {
                position: Vector3::new(-0.5, 0.0, 0.8),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 0.7,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -0.7, 0.0),
                        max_force: 500.0,
                        activation: 0.0,
                    },
                ],
            },
            // Front right leg
            Limb {
                position: Vector3::new(0.5, 0.0, 0.8),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 0.7,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -0.7, 0.0),
                        max_force: 500.0,
                        activation: 0.0,
                    },
                ],
            },
            // Middle left leg
            Limb {
                position: Vector3::new(-0.5, 0.0, 0.0),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 0.7,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -0.7, 0.0),
                        max_force: 500.0,
                        activation: 0.0,
                    },
                ],
            },
            // Middle right leg
            Limb {
                position: Vector3::new(0.5, 0.0, 0.0),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 0.7,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -0.7, 0.0),
                        max_force: 500.0,
                        activation: 0.0,
                    },
                ],
            },
            // Back left leg
            Limb {
                position: Vector3::new(-0.5, 0.0, -0.8),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 0.7,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -0.7, 0.0),
                        max_force: 500.0,
                        activation: 0.0,
                    },
                ],
            },
            // Back right leg
            Limb {
                position: Vector3::new(0.5, 0.0, -0.8),
                rotation: Vector3::new(0.0, 0.0, 0.0),
                length: 0.7,
                muscles: vec![
                    Muscle {
                        origin: Vector3::new(0.0, 0.0, 0.0),
                        insertion: Vector3::new(0.0, -0.7, 0.0),
                        max_force: 500.0,
                        activation: 0.0,
                    },
                ],
            },
        ]
    }

    pub fn update(&mut self, dt: f32, forces: &[Vector3<f32>]) {
        // Apply external forces (muscles, gravity, etc.)
        let mut net_force = Vector3::new(0.0, -9.81 * self.mass, 0.0); // Gravity
        for force in forces {
            net_force += force;
        }

        // Calculate acceleration F = ma -> a = F/m
        self.acceleration = net_force / self.mass;

        // Update velocity v = v0 + a*t
        self.velocity += self.acceleration * dt;

        // Update position p = p0 + v*t
        self.position += self.velocity * dt;

        // Keep creature above ground (simple collision)
        if self.position.y < 1.0 {
            self.position.y = 1.0;
            self.velocity.y = 0.0;
            self.acceleration.y = 0.0;
        }
    }

    pub fn apply_muscle_activation(&mut self, activations: &[f32]) {
        for (i, limb) in self.limbs.iter_mut().enumerate() {
            if i < activations.len() {
                for muscle in &mut limb.muscles {
                    muscle.activation = activations[i];
                }
            }
        }
    }

    pub fn get_limb_positions(&self) -> Vec<(Vector3<f32>, Vector3<f32>)> {
        self.limbs
            .iter()
            .map(|limb| (limb.position + self.position, limb.rotation))
            .collect()
    }
}

// WASM interface functions
#[wasm_bindgen]
pub struct GameWorld {
    creatures: Vec<Creature>,
    terrain_heights: HashMap<(i32, i32), f32>,
}

#[wasm_bindgen]
impl GameWorld {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameWorld {
        // Set the panic hook for better error reporting in debug mode
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        GameWorld {
            creatures: vec![],
            terrain_heights: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_creature(&mut self, morphology: u8) -> usize {
        let morph = match morphology {
            0 => Morphology::Biped,
            1 => Morphology::Quadruped,
            2 => Morphology::Hexapod,
            _ => Morphology::Biped, // Default to biped
        };

        let creature = Creature::new(morph);
        self.creatures.push(creature);
        self.creatures.len() - 1
    }

    #[wasm_bindgen]
    pub fn update_creature(&mut self, idx: usize, dt: f32, forces_json: &str) {
        if idx >= self.creatures.len() {
            return;
        }

        // Parse JSON string to vector of forces
        if let Ok(forces) = serde_json::from_str::<Vec<[f32; 3]>>(forces_json) {
            let force_vectors: Vec<Vector3<f32>> = forces
                .iter()
                .map(|f| Vector3::new(f[0], f[1], f[2]))
                .collect();
            
            self.creatures[idx].update(dt, &force_vectors);
        }
    }

    #[wasm_bindgen]
    pub fn activate_muscles(&mut self, idx: usize, activations_json: &str) {
        if idx >= self.creatures.len() {
            return;
        }

        // Parse JSON string to vector of activations
        if let Ok(activations) = serde_json::from_str::<Vec<f32>>(activations_json) {
            self.creatures[idx].apply_muscle_activation(&activations);
        }
    }

    #[wasm_bindgen]
    pub fn get_creature_position(&self, idx: usize) -> JsValue {
        if idx >= self.creatures.len() {
            return JsValue::NULL;
        }

        let pos = &self.creatures[idx].position;
        JsValue::from_serde(&[pos.x, pos.y, pos.z]).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn get_creature_rotation(&self, idx: usize) -> JsValue {
        if idx >= self.creatures.len() {
            return JsValue::NULL;
        }

        let rot = &self.creatures[idx].rotation;
        JsValue::from_serde(&[rot.x, rot.y, rot.z]).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn get_limb_positions(&self, idx: usize) -> JsValue {
        if idx >= self.creatures.len() {
            return JsValue::NULL;
        }

        let positions = self.creatures[idx].get_limb_positions();
        let js_positions: Vec<f32> = positions
            .into_iter()
            .flat_map(|(pos, rot)| [pos.x, pos.y, pos.z, rot.x, rot.y, rot.z])
            .collect();
        
        JsValue::from_serde(&js_positions).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn get_creature_velocity(&self, idx: usize) -> JsValue {
        if idx >= self.creatures.len() {
            return JsValue::NULL;
        }

        let vel = &self.creatures[idx].velocity;
        JsValue::from_serde(&[vel.x, vel.y, vel.z]).unwrap_or(JsValue::NULL)
    }

    #[wasm_bindgen]
    pub fn set_terrain_height(&mut self, x: i32, z: i32, height: f32) {
        self.terrain_heights.insert((x, z), height);
    }

    #[wasm_bindgen]
    pub fn get_terrain_height(&self, x: i32, z: i32) -> f32 {
        self.terrain_heights.get(&(x, z)).copied().unwrap_or(0.0)
    }
}

// Helper function to initialize a random seed
#[wasm_bindgen]
pub fn init_random_seed(seed: u64) {
    // In a real implementation, we would use the seed to initialize a random generator
    // For now, we just acknowledge the seed value
    let _seed = seed;
}