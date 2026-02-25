use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Morphology {
    Biped = 0,
    Quadruped = 1,
    Hexapod = 2,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Muscle {
    pub max_force: f32,
    pub activation: f32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Limb {
    pub length: f32,
    pub muscles: Vec<Muscle>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Creature {
    pub morphology: Morphology,
    pub mass: f32,
    pub limbs: Vec<Limb>,
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
            mass,
            limbs,
        }
    }

    fn create_biped_limbs() -> Vec<Limb> {
        vec![
            Limb { length: 1.0, muscles: vec![Muscle { max_force: 800.0, activation: 0.0 }] },
            Limb { length: 1.0, muscles: vec![Muscle { max_force: 800.0, activation: 0.0 }] },
        ]
    }

    fn create_quadruped_limbs() -> Vec<Limb> {
        vec![
            Limb { length: 0.8, muscles: vec![Muscle { max_force: 600.0, activation: 0.0 }] },
            Limb { length: 0.8, muscles: vec![Muscle { max_force: 600.0, activation: 0.0 }] },
            Limb { length: 0.8, muscles: vec![Muscle { max_force: 600.0, activation: 0.0 }] },
            Limb { length: 0.8, muscles: vec![Muscle { max_force: 600.0, activation: 0.0 }] },
        ]
    }

    fn create_hexapod_limbs() -> Vec<Limb> {
        vec![
            Limb { length: 0.7, muscles: vec![Muscle { max_force: 500.0, activation: 0.0 }] },
            Limb { length: 0.7, muscles: vec![Muscle { max_force: 500.0, activation: 0.0 }] },
            Limb { length: 0.7, muscles: vec![Muscle { max_force: 500.0, activation: 0.0 }] },
            Limb { length: 0.7, muscles: vec![Muscle { max_force: 500.0, activation: 0.0 }] },
            Limb { length: 0.7, muscles: vec![Muscle { max_force: 500.0, activation: 0.0 }] },
            Limb { length: 0.7, muscles: vec![Muscle { max_force: 500.0, activation: 0.0 }] },
        ]
    }
}
