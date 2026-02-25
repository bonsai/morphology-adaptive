// Three.js variables
let scene, camera, renderer;
let playerMorph, track, terrainBlocks = [];
let clock;

// Backend detection and Game Logic abstraction
let gameLogic;
let isPythonBackend = false;

// Game state (JS-side for UI only)
let uiState = {
    totalLaps: 3,
    morphType: 'Biped'
};

// Input handling
let keys = {};

// Initialize the game
function init() {
    // Determine backend
    if (window.pyodide && window.GameState) {
        console.log("Using Python (Pyodide) backend");
        gameLogic = window.GameState(uiState.totalLaps);
        isPythonBackend = true;
    } else if (window.GameState) {
        console.log("Using Rust (WASM) backend");
        const morphIdx = uiState.morphType === 'Biped' ? 0 : (uiState.morphType === 'Quadruped' ? 1 : 2);
        gameLogic = new window.GameState(uiState.totalLaps, morphIdx);
        isPythonBackend = false;
    } else {
        console.error("No GameState backend (Rust or Python) loaded");
        return;
    }

    clock = new THREE.Clock();
    // Create scene
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x87CEEB); // Sky blue
    
    // Add lighting
    const ambientLight = new THREE.AmbientLight(0x606060);
    scene.add(ambientLight);
    
    const directionalLight = new THREE.DirectionalLight(0xffffff, 1);
    directionalLight.position.set(1, 1, 0.5).normalize();
    scene.add(directionalLight);

    // Create camera
    camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    camera.position.set(0, 5, 10);
    camera.lookAt(0, 0, 0);

    // Create renderer
    renderer = new THREE.WebGLRenderer({ canvas: document.getElementById('gameCanvas') });
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.setPixelRatio(window.devicePixelRatio);

    // Create the racetrack (simplified as a circular path)
    createTrack();

    // Create the player morph (initially a biped)
    createPlayerMorph();

    // Set up event listeners
    setupEventListeners();

    // Start the game loop
    animate();
}

function createTrack() {
    // Create a circular track using a torus knot as the base path
    const trackGeometry = new THREE.TorusKnotGeometry(10, 0.5, 128, 32, 2, 3);
    const trackMaterial = new THREE.MeshPhongMaterial({ color: 0x303030, wireframe: false });
    track = new THREE.Mesh(trackGeometry, trackMaterial);
    scene.add(track);

    // Add some terrain blocks to represent different terrains
    for (let i = 0; i < 10; i++) {
        const angle = (i / 10) * Math.PI * 2;
        const radius = 10;
        const x = Math.cos(angle) * radius;
        const z = Math.sin(angle) * radius;
        
        const geometry = new THREE.BoxGeometry(2, 0.5, 2);
        const material = new THREE.MeshPhongMaterial({ 
            color: i % 2 === 0 ? 0x8B4513 : 0x228B22 // Alternating dirt and grass
        });
        const block = new THREE.Mesh(geometry, material);
        block.position.set(x, -0.5, z);
        scene.add(block);
        terrainBlocks.push(block);
    }
}

function createPlayerMorph() {
    // Create a simple representation of a biped creature
    const group = new THREE.Group();
    
    // Body
    const bodyGeometry = new THREE.CapsuleGeometry(0.5, 1, 4, 8);
    const bodyMaterial = new THREE.MeshPhongMaterial({ color: 0x00ff00 });
    const body = new THREE.Mesh(bodyGeometry, bodyMaterial);
    body.position.y = 1.5;
    group.add(body);
    
    // Head
    const headGeometry = new THREE.SphereGeometry(0.5, 16, 16);
    const headMaterial = new THREE.MeshPhongMaterial({ color: 0xffff00 });
    const head = new THREE.Mesh(headGeometry, headMaterial);
    head.position.y = 2.5;
    group.add(head);
    
    // Legs (simplified)
    const legGeometry = new THREE.CapsuleGeometry(0.2, 0.8, 4, 8);
    const legMaterial = new THREE.MeshPhongMaterial({ color: 0x0000ff });
    
    const leftLeg = new THREE.Mesh(legGeometry, legMaterial);
    leftLeg.position.set(-0.3, 0.5, 0);
    group.add(leftLeg);
    
    const rightLeg = new THREE.Mesh(legGeometry, legMaterial);
    rightLeg.position.set(0.3, 0.5, 0);
    group.add(rightLeg);
    
    // Position the creature at the start of the track
    group.position.set(10, 1, 0);
    scene.add(group);
    
    playerMorph = group;
}

function setupEventListeners() {
    // Keyboard event listeners
    window.addEventListener('keydown', (event) => {
        keys[event.code] = true;
    });
    
    window.addEventListener('keyup', (event) => {
        keys[event.code] = false;
    });
    
    // Button event listeners
    document.getElementById('startButton').addEventListener('click', startRace);
    document.getElementById('restartButton').addEventListener('click', restartGame);
    
    // Handle window resize
    window.addEventListener('resize', () => {
        camera.aspect = window.innerWidth / window.innerHeight;
        camera.updateProjectionMatrix();
        renderer.setSize(window.innerWidth, window.innerHeight);
    });
}

function startRace() {
    gameLogic.start_race(Date.now());
    document.getElementById('startScreen').style.display = 'none';
}

function restartGame() {
    if (isPythonBackend) {
        gameLogic = window.GameState(uiState.totalLaps);
    } else {
        const morphIdx = uiState.morphType === 'Biped' ? 0 : (uiState.morphType === 'Quadruped' ? 1 : 2);
        gameLogic = new window.GameState(uiState.totalLaps, morphIdx);
    }
    playerMorph.position.set(10, 1, 0);
    playerMorph.rotation.set(0, 0, 0);
    document.getElementById('endScreen').style.display = 'none';
    startRace();
}

function updateGameState() {
    const delta = clock.getDelta();
    const activeKeys = Object.keys(keys).filter(k => keys[k]);
    
    gameLogic.update(delta, Date.now(), activeKeys);
    
    // Update player position and rotation from Python
    playerMorph.position.set(gameLogic.get_x(), gameLogic.get_y(), gameLogic.get_z());
    playerMorph.rotation.y = gameLogic.get_rotation_y();
    
    // Update timer UI
    document.getElementById('timer').textContent = gameLogic.get_current_time().toFixed(2) + 's';
    
    // Update speed UI
    document.getElementById('speed').textContent = Math.abs(gameLogic.get_speed() * 3.6).toFixed(1) + ' km/h';
    
    // Update morph type UI
    document.getElementById('morphType').textContent = uiState.morphType;
    
    // Update lap counter UI
    document.getElementById('lapCounter').textContent = `${gameLogic.get_lap()}/${uiState.totalLaps}`;
    
    // Check for race completion
    if (gameLogic.is_completed()) {
        document.getElementById('finalTime').textContent = gameLogic.get_current_time().toFixed(2) + 's';
        document.getElementById('endScreen').style.display = 'flex';
    }

    // Camera follow
    const relativeCameraOffset = new THREE.Vector3(0, 5, -10);
    const cameraOffset = relativeCameraOffset.applyMatrix4(playerMorph.matrixWorld);
    camera.position.x = cameraOffset.x;
    camera.position.y = cameraOffset.y;
    camera.position.z = cameraOffset.z;
    camera.lookAt(playerMorph.position);
}

function animate() {
    requestAnimationFrame(animate);
    
    updateGameState();
    
    // Rotate the track slightly for visual effect
    if (track) {
        track.rotation.y += 0.001;
    }
    
    // Render the scene
    renderer.render(scene, camera);
}

// Start the game when the page loads
window.onload = init;