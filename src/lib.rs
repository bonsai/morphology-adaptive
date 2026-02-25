use wasm_bindgen::prelude::*;
use std::f32::consts::PI;

#[wasm_bindgen]
pub struct GameState {
    x: f32,
    y: f32,
    z: f32,
    rotation_y: f32,
    speed: f32,
    lap: i32,
    total_laps: i32,
    last_angle: f32,
    total_angle: f32,
    race_started: bool,
    race_completed: bool,
    start_time: f64,
    current_time: f64,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new(total_laps: i32) -> GameState {
        GameState {
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

    pub fn start_race(&mut self, now: f64) {
        self.race_started = true;
        self.start_time = now;
        self.last_angle = self.z.atan2(self.x);
        self.total_angle = 0.0;
    }

    pub fn update(&mut self, delta: f32, now: f64, keys: Vec<String>) {
        if !self.race_started || self.race_completed {
            return;
        }

        self.current_time = (now - self.start_time) / 1000.0;

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

        // Calculate movement based on rotation
        // In script.js: playerMorph.translateZ(delta * speed)
        // translateZ in Three.js moves along the local Z axis.
        // Assuming the model faces -Z (default for Three.js Group)
        // If it faces +Z, then x += sin(rot) * speed, z += cos(rot) * speed
        // Let's match script.js's translateZ(delta * speed)
        // For a simple y-rotation, translateZ(d) means:
        // dx = sin(rotation_y) * d
        // dz = cos(rotation_y) * d
        
        let distance = delta * self.speed;
        self.x += self.rotation_y.sin() * distance;
        self.z += self.rotation_y.cos() * distance;

        // Lap counting
        let current_angle = self.z.atan2(self.x);
        let mut angle_diff = current_angle - self.last_angle;

        if angle_diff > PI {
            angle_diff -= PI * 2.0;
        }
        if angle_diff < -PI {
            angle_diff += PI * 2.0;
        }

        self.total_angle += angle_diff;
        self.last_angle = current_angle;

        let laps_completed = (self.total_angle.abs() / (PI * 2.0)).floor() as i32;
        if laps_completed > self.lap {
            self.lap = laps_completed;
        }

        if self.lap >= self.total_laps {
            self.race_completed = true;
        }
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
