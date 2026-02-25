import math

class GameState:
    def __init__(self, total_laps):
        self.x = 10.0
        self.y = 1.0
        self.z = 0.0
        self.rotation_y = 0.0
        self.speed = 0.0
        self.lap = 0
        self.total_laps = total_laps
        self.last_angle = 0.0
        self.total_angle = 0.0
        self.race_started = False
        self.race_completed = False
        self.start_time = 0.0
        self.current_time = 0.0

    def start_race(self, now):
        self.race_started = True
        self.start_time = now
        self.last_angle = math.atan2(self.z, self.x)
        self.total_angle = 0.0

    def update(self, delta, now, keys_json):
        if not self.race_started or self.race_completed:
            return

        self.current_time = (now - self.start_time) / 1000.0

        base_speed = 15.0
        rotation_speed = 3.0

        # Convert JsProxy to list if needed
        keys = list(keys_json) if hasattr(keys_json, 'to_py') else keys_json

        move_forward = any(k in ["ArrowUp", "KeyW"] for k in keys)
        move_backward = any(k in ["ArrowDown", "KeyS", "Space"] for k in keys)
        turn_left = any(k in ["ArrowLeft", "KeyA"] for k in keys)
        turn_right = any(k in ["ArrowRight", "KeyD"] for k in keys)


        if move_forward:
            self.speed = base_speed
        elif move_backward:
            self.speed = -base_speed * 0.5
        else:
            self.speed *= 0.95

        if turn_left:
            self.rotation_y += delta * rotation_speed
        if turn_right:
            self.rotation_y -= delta * rotation_speed

        distance = delta * self.speed
        self.x += math.sin(self.rotation_y) * distance
        self.z += math.cos(self.rotation_y) * distance

        # Lap counting
        current_angle = math.atan2(self.z, self.x)
        angle_diff = current_angle - self.last_angle

        if angle_diff > math.pi:
            angle_diff -= math.pi * 2.0
        if angle_diff < -math.pi:
            angle_diff += math.pi * 2.0

        self.total_angle += angle_diff
        self.last_angle = current_angle

        laps_completed = int(abs(self.total_angle) / (math.pi * 2.0))
        if laps_completed > self.lap:
            self.lap = laps_completed

        if self.lap >= self.total_laps:
            self.race_completed = True

    def get_x(self): return self.x
    def get_y(self): return self.y
    def get_z(self): return self.z
    def get_rotation_y(self): return self.rotation_y
    def get_speed(self): return self.speed
    def get_lap(self): return self.lap
    def is_completed(self): return self.race_completed
    def get_current_time(self): return self.current_time
