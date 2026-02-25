# build.ps1 - Windows Build Preparation Script (Hybrid Rust/Python)

$ProjectRoot = $PSScriptRoot

Write-Host "--- Environment Check (Hybrid) ---" -ForegroundColor Cyan

# 0. Check for tools
$hasDocker = Get-Command docker -ErrorAction SilentlyContinue

# 1. Check for required files
$LogicFile = Join-Path $ProjectRoot "game_logic.py"
if (Test-Path $LogicFile) {
    Write-Host "[OK] game_logic.py found." -ForegroundColor Green
} else {
    Write-Error "game_logic.py not found at $LogicFile"
    exit 1
}

# 2. Build Strategy
Write-Host "`n--- Build Strategy ---" -ForegroundColor Cyan

if ($hasDocker) {
    Write-Host "[Info] Docker detected. Building Rust WASM via Docker (High Performance)." -ForegroundColor Green
    Write-Host "Building container image..."
    docker build -t morphology-builder $ProjectRoot
    Write-Host "Compiling Rust to WASM..."
    docker run --rm -v "${ProjectRoot}:/build" -w /build morphology-builder wasm-pack build --target web --out-dir pkg
} else {
    Write-Host "[Info] Docker not found. Using Python/Pyodide (No compilation needed)." -ForegroundColor Yellow
    Write-Host "The project will automatically fallback to Python logic in the browser." -ForegroundColor Gray
}

# 3. Instructions
Write-Host "`n--- Ready ---" -ForegroundColor Green
Write-Host "The project is configured for Hybrid Backend:"
Write-Host "1. Rust WASM (Fastest, loaded from pkg/)"
Write-Host "2. Python/Pyodide (Fallback, loaded from game_logic.py)"
Write-Host "`nRun the following command to start the server (at project root):" -ForegroundColor Yellow
Write-Host "cd $ProjectRoot"
Write-Host "python -m http.server 8000"
Write-Host "`nThen open this URL in your browser:" -ForegroundColor Yellow
Write-Host "http://localhost:8000/index.html"

