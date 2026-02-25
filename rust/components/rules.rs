use crate::components::state::GameState;
use std::f32::consts::PI;

pub struct RaceRules;

impl RaceRules {
    pub fn start(state: &mut GameState, now: f64) {
        state.race_started = true;
        state.start_time = now;
        state.last_angle = state.z.atan2(state.x);
        state.total_angle = 0.0;
    }

    pub fn update(state: &mut GameState, now: f64) {
        if !state.race_started || state.race_completed {
            return;
        }

        state.current_time = (now - state.start_time) / 1000.0;

        // Lap counting
        let current_angle = state.z.atan2(state.x);
        let mut angle_diff = current_angle - state.last_angle;

        if angle_diff > PI {
            angle_diff -= PI * 2.0;
        }
        if angle_diff < -PI {
            angle_diff += PI * 2.0;
        }

        state.total_angle += angle_diff;
        state.last_angle = current_angle;

        let laps_completed = (state.total_angle.abs() / (PI * 2.0)).floor() as i32;
        if laps_completed > state.lap {
            state.lap = laps_completed;
        }

        if state.lap >= state.total_laps {
            state.race_completed = true;
        }
    }
}
