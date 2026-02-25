use crate::components::state::GameState;

pub struct PhysicsEngine;

impl PhysicsEngine {
    pub fn update(state: &mut GameState, delta: f32, keys: &Vec<String>) {
        if !state.race_started || state.race_completed {
            return;
        }

        let base_speed = 15.0;
        let rotation_speed = 3.0;

        let mut move_forward = false;
        let mut move_backward = false;
        let mut turn_left = false;
        let mut turn_right = false;

        for key in keys {
            match key.as_str() {
                "ArrowUp" | "KeyW" => move_forward = true,
                "ArrowDown" | "KeyS" | "Space" => move_backward = true,
                "ArrowLeft" | "KeyA" => turn_left = true,
                "ArrowRight" | "KeyD" => turn_right = true,
                _ => {}
            }
        }

        if move_forward {
            state.speed = base_speed;
        } else if move_backward {
            state.speed = -base_speed * 0.5;
        } else {
            state.speed *= 0.95;
        }

        if turn_left {
            state.rotation_y += delta * rotation_speed;
        }
        if turn_right {
            state.rotation_y -= delta * rotation_speed;
        }

        let distance = delta * state.speed;
        state.x += state.rotation_y.sin() * distance;
        state.z += state.rotation_y.cos() * distance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::state::GameState;
    use crate::components::creature::Morphology;

    #[test]
    fn test_physics_forward() {
        let mut state = GameState::new(3, Morphology::Biped);
        state.race_started = true;
        
        let keys = vec!["ArrowUp".to_string()];
        PhysicsEngine::update(&mut state, 0.1, &keys);
        
        assert_eq!(state.speed, 15.0);
        assert!(state.z > 0.0);
        assert_eq!(state.x, 10.0);
    }

    #[test]
    fn test_physics_turn() {
        let mut state = GameState::new(3, Morphology::Biped);
        state.race_started = true;
        
        let keys = vec!["ArrowLeft".to_string()];
        PhysicsEngine::update(&mut state, 0.1, &keys);
        
        assert!(state.rotation_y > 0.0);
    }
}

