# ADR 001: Current Architecture & Status

## Status
Accepted

## Context
We are developing a morphology-adaptive racing game. The project has evolved from a simple HTML/JS prototype to a more robust architecture involving Rust (WASM) and Python (FastAPI).

## Decision
We have adopted a hybrid architecture:
1.  **Frontend**: HTML5 + Three.js for rendering.
2.  **Game Logic**:
    *   **Primary**: Rust compiled to WebAssembly (WASM) for high performance in the browser.
    *   **Fallback**: Python logic running in the browser via Pyodide (for environments where WASM might fail or for rapid prototyping).
    *   **Backend**: Python logic running on the server (FastAPI) for validation, state management, and potential multiplayer features.
3.  **Backend Framework**: FastAPI (Python) wrapped in Docker.
    *   Serves the frontend static files.
    *   Provides REST API endpoints (`/start`, `/update`, `/state`) for game state management.

## Current Implementation Details
-   **Docker**: The application is containerized. The Dockerfile installs Rust, Python, Node.js, and builds the environment.
-   **FastAPI**: `main.py` serves the game and API.
-   **Game Logic Synchronization**: `game_logic.py` (Python) and `src/lib.rs` (Rust) maintain equivalent logic for 2-player racing.

## Known Issues & Next Steps
1.  **WASM Build in Docker**: The Docker image currently does not run `wasm-pack build`, causing 404 errors for `.wasm` files.
    *   *Fix*: Update Dockerfile to build WASM artifacts.
2.  **Static File Serving**: The FastAPI app needs to be configured to serve the `pkg/` directory containing WASM artifacts.
    *   *Fix*: Update `main.py` to mount `pkg/`.
3.  **Frontend Logic**: `script.js` currently runs independent game logic. Integration with the Backend API is available but not enforced for the main loop (to avoid network latency).

## Consequences
-   **Pros**: Flexible deployment (Docker), high-performance client-side logic (WASM), standard API for backend integration.
-   **Cons**: distinct logic implementations (Rust vs Python) must be kept in sync.
