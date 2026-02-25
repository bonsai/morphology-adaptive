import math

class GameState:
    def __init__(self, morphology_idx):
        # Creature 1 (top lane, facing right)
        self.creature1_x = -10.0
        self.creature1_y = 1.0
        self.creature1_z = 2.0  # Top lane
        self.creature1_rotation_y = 0.0
        self.creature1_speed = 0.0
        
        # Creature 2 (bottom lane, facing right)
        self.creature2_x = -10.0
        self.creature2_y = 1.0
        self.creature2_z = -2.0  # Bottom lane
        self.creature2_rotation_y = 0.0
        self.creature2_speed = 0.0
        
        # Racing game state
        self.game_started = False
        self.game_completed = False
        self.start_time = 0.0
        self.current_time = 0.0
        self.winner = 0

    def start_game(self, now):
        self.game_started = True
        self.start_time = now
        self.winner = 0

    def update(self, delta, now, keys_json):
        if not self.game_started or self.game_completed:
            return

        self.current_time = (now - self.start_time) / 1000.0

        # Convert JsProxy to list if needed
        keys = list(keys_json) if hasattr(keys_json, 'to_py') else keys_json

        # Racing physics constants
        acceleration = 15.0
        max_speed = 30.0
        friction = 0.95
        finish_line_x = 20.0

        # Player 1 controls (WASD)
        p1_accelerate = any(k == "KeyW" for k in keys)
        p1_brake = any(k == "KeyS" for k in keys)
        
        # Player 2 controls (Arrow keys)
        p2_accelerate = any(k == "ArrowUp" for k in keys)
        p2_brake = any(k == "ArrowDown" for k in keys)

        # Update creature 1 speed
        if p1_accelerate:
            self.creature1_speed += acceleration * delta
        elif p1_brake:
            self.creature1_speed -= acceleration * delta * 2.0
        
        self.creature1_speed = max(0.0, min(self.creature1_speed, max_speed))
        self.creature1_speed *= friction

        # Update creature 2 speed
        if p2_accelerate:
            self.creature2_speed += acceleration * delta
        elif p2_brake:
            self.creature2_speed -= acceleration * delta * 2.0
            
        self.creature2_speed = max(0.0, min(self.creature2_speed, max_speed))
        self.creature2_speed *= friction

        # Update positions
        self.creature1_x += self.creature1_speed * delta
        self.creature2_x += self.creature2_speed * delta

        # Check win conditions
        if not self.game_completed:
            if self.creature1_x > finish_line_x and self.creature2_x > finish_line_x:
                self.game_completed = True
                self.winner = 1 if self.creature1_x > self.creature2_x else 2
            elif self.creature1_x > finish_line_x:
                self.game_completed = True
                self.winner = 1
            elif self.creature2_x > finish_line_x:
                self.game_completed = True
                self.winner = 2

    # Getters matching Rust interface
    def get_creature1_x(self): return self.creature1_x
    def get_creature1_y(self): return self.creature1_y
    def get_creature1_z(self): return self.creature1_z
    def get_creature1_rotation_y(self): return self.creature1_rotation_y
    def get_creature1_speed(self): return self.creature1_speed
    
    def get_creature2_x(self): return self.creature2_x
    def get_creature2_y(self): return self.creature2_y
    def get_creature2_z(self): return self.creature2_z
    def get_creature2_rotation_y(self): return self.creature2_rotation_y
    def get_creature2_speed(self): return self.creature2_speed
    
    def is_completed(self): return self.game_completed
    def get_winner(self): return self.winner
    def get_current_time(self): return self.current_time
