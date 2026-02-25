from fastapi import FastAPI, HTTPException
from fastapi.staticfiles import StaticFiles
from starlette.responses import FileResponse
from pydantic import BaseModel
from typing import List, Optional
import time
import os
from game_logic import GameState

app = FastAPI()

# Global game state (for demo purposes)
game_state = GameState(0)

class GameInput(BaseModel):
    keys: List[str]
    delta: float

class GameStatus(BaseModel):
    creature1_x: float
    creature1_y: float
    creature1_z: float
    creature1_speed: float
    creature2_x: float
    creature2_y: float
    creature2_z: float
    creature2_speed: float
    game_started: bool
    game_completed: bool
    winner: int
    current_time: float

@app.get("/health")
def health_check():
    return {"status": "ok"}

@app.post("/start")
def start_game():
    global game_state
    now = time.time() * 1000  # JS uses milliseconds
    game_state = GameState(0) # Reset
    game_state.start_game(now)
    return {"message": "Game started", "start_time": now}

@app.post("/update")
def update_game(input_data: GameInput):
    now = time.time() * 1000
    game_state.update(input_data.delta, now, input_data.keys)
    return get_state()

@app.get("/state", response_model=GameStatus)
def get_state():
    return GameStatus(
        creature1_x=game_state.get_creature1_x(),
        creature1_y=game_state.get_creature1_y(),
        creature1_z=game_state.get_creature1_z(),
        creature1_speed=game_state.get_creature1_speed(),
        creature2_x=game_state.get_creature2_x(),
        creature2_y=game_state.get_creature2_y(),
        creature2_z=game_state.get_creature2_z(),
        creature2_speed=game_state.get_creature2_speed(),
        game_started=game_state.game_started,
        game_completed=game_state.is_completed(),
        winner=game_state.get_winner(),
        current_time=game_state.get_current_time()
    )

# Serve static files
@app.get("/")
async def read_index():
    return FileResponse('index.html')

@app.get("/script.js")
async def read_script():
    return FileResponse('script.js')

@app.get("/game_logic.py")
async def read_game_logic():
    return FileResponse('game_logic.py')

if os.path.exists("pkg"):
    app.mount("/pkg", StaticFiles(directory="pkg"), name="pkg")

if os.path.exists("data"):
    app.mount("/data", StaticFiles(directory="data"), name="data")

if os.path.exists("src"):
    app.mount("/src", StaticFiles(directory="src"), name="src")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
