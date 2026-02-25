// Three.js variables
let scene, camera, renderer;
let playerMorph1, playerMorph2, ropeMesh, arena;
let clock;
let creatureMeshData = null;
let nodeMeshes = [];
let edgeLines = [];

// Backend detection and Game Logic abstraction
let gameLogic;
let isPythonBackend = false;

// Game state (JS-side for UI only)
let uiState = {
    morphType: 'Biped'
};

// Input handling
let keys = {};

// Initialize the game
function init() {
    console.log("Initializing game...");
    // Determine backend
    if (window.pyodide && window.GameState) {
        console.log("Using Python (Pyodide) backend");
        try {
            const morphIdx = uiState.morphType === 'Biped' ? 0 : (uiState.morphType === 'Quadruped' ? 1 : 2);
            gameLogic = window.GameState(morphIdx);
            isPythonBackend = true;
            console.log("Python GameState instance created");
        } catch (e) {
            console.error("Failed to create Python GameState:", e);
        }
    } else if (window.GameState) {
        console.log("Using Rust (WASM) backend");
        try {
            if (window.gameStateInstance) {
                gameLogic = window.gameStateInstance;
                console.log("Using pre-initialized Rust GameState with policy");
            } else {
                const morphIdx = uiState.morphType === 'Biped' ? 0 : (uiState.morphType === 'Quadruped' ? 1 : 2);
                gameLogic = new window.GameState(morphIdx);
            }
            isPythonBackend = false;
            console.log("Rust GameState instance created");
        } catch (e) {
            console.error("Failed to create Rust GameState:", e);
        }
    } else {
        console.error("No GameState backend (Rust or Python) loaded. Check console for previous errors.");
        return;
    }

    if (!gameLogic) {
        console.error("gameLogic failed to initialize");
        return;
    }

    // Load creature mesh data
    loadCreatureMesh(uiState.morphType);

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

    // Create the tug-of-war arena
    createArena();

    // Create both player morphs for tug-of-war
    createPlayerMorphs();

    // Set up event listeners
    setupEventListeners();

    // Start the game loop
    animate();
}

function createArena() {
    // Create racing track with two lanes
    const arenaGeometry = new THREE.PlaneGeometry(60, 8);
    const arenaMaterial = new THREE.MeshPhongMaterial({ color: 0x228822 });
    arena = new THREE.Mesh(arenaGeometry, arenaMaterial);
    arena.rotation.x = -Math.PI / 2;
    arena.position.y = -0.05;
    scene.add(arena);

    // Create lane dividers
    const laneDividerGeometry = new THREE.PlaneGeometry(0.1, 60);
    const laneDividerMaterial = new THREE.MeshBasicMaterial({ color: 0xffffff });
    
    const laneDivider1 = new THREE.Mesh(laneDividerGeometry, laneDividerMaterial);
    laneDivider1.rotation.x = -Math.PI / 2;
    laneDivider1.position.set(0, 0.01, 1.0);
    scene.add(laneDivider1);
    
    const laneDivider2 = new THREE.Mesh(laneDividerGeometry, laneDividerMaterial);
    laneDivider2.rotation.x = -Math.PI / 2;
    laneDivider2.position.set(0, 0.01, -1.0);
    scene.add(laneDivider2);

    // Create finish line
    const finishLineGeometry = new THREE.PlaneGeometry(8, 1.0);
    const finishLineMaterial = new THREE.MeshBasicMaterial({ 
        color: 0xffffff, 
        transparent: true, 
        opacity: 0.7 
    });
    const finishLine = new THREE.Mesh(finishLineGeometry, finishLineMaterial);
    finishLine.rotation.x = -Math.PI / 2;
    finishLine.position.set(20, 0.02, 0);
    scene.add(finishLine);
}

function createPlayerMorphs() {
    // Create two creatures for racing
    
    // Creature 1 (Player 1, top lane, blue)
    playerMorph1 = createSimpleCreature(0x0000ff);
    playerMorph1.position.set(-10, 1, 2); // Top lane
    scene.add(playerMorph1);
    
    // Creature 2 (Player 2, bottom lane, red) 
    playerMorph2 = createSimpleCreature(0xff0000);
    playerMorph2.position.set(-10, 1, -2); // Bottom lane
    scene.add(playerMorph2);
}

function createSimpleCreature(color) {
    const group = new THREE.Group();
    
    // Body
    const bodyGeometry = new THREE.CapsuleGeometry(0.5, 1, 4, 8);
    const bodyMaterial = new THREE.MeshPhongMaterial({ color: color });
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
    const legMaterial = new THREE.MeshPhongMaterial({ color: color });
    
    const leftLeg = new THREE.Mesh(legGeometry, legMaterial);
    leftLeg.position.set(-0.3, 0.5, 0);
    group.add(leftLeg);
    
    const rightLeg = new THREE.Mesh(legGeometry, legMaterial);
    rightLeg.position.set(0.3, 0.5, 0);
    group.add(rightLeg);
    
    return group;
}

async function loadCreatureMesh(type) {
    const path = `data/agents/${type.toLowerCase()}/mesh.json`;
    try {
        const response = await fetch(path);
        const meshJsonText = await response.text();
        creatureMeshData = JSON.parse(meshJsonText);
        console.log(`Loaded mesh for ${type}`);
        
        // Initialize Rust simulation with this mesh
        if (gameLogic && !isPythonBackend) {
            gameLogic.init_simulation(meshJsonText);
            console.log("Rust SoftBody simulation initialized");
        }

        // Replace current player morph with complex one
        if (playerMorph) {
            scene.remove(playerMorph);
            createComplexCreature(creatureMeshData);
        }
    } catch (e) {
        console.error(`Failed to load mesh for ${type}:`, e);
    }
}

function createComplexCreature(data) {
    const group = new THREE.Group();
    nodeMeshes = [];
    edgeLines = [];

    const nodes = data.pos;
    const triangles = data.triangles;

    // Find min Y to ground the creature
    let minY = Infinity;
    nodes.forEach(pos => {
        if (pos[1] < minY) minY = pos[1];
    });

    // Create a mesh for the body triangles
    const geometry = new THREE.BufferGeometry();
    const vertices = new Float32Array(nodes.length * 3);
    const indices = [];

    nodes.forEach((pos, i) => {
        vertices[i * 3] = pos[0];
        vertices[i * 3 + 1] = pos[1] - minY; // Ground it
        vertices[i * 3 + 2] = 0;
    });

    triangles.forEach(tri => {
        indices.push(tri[0], tri[1], tri[2]);
    });

    geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
    geometry.setIndex(indices);
    geometry.computeVertexNormals();

    const material = new THREE.MeshPhongMaterial({ 
        color: 0x00ff00, 
        side: THREE.DoubleSide,
        transparent: true,
        opacity: 0.8
    });
    const mesh = new THREE.Mesh(geometry, material);
    group.add(mesh);

    // Create spheres for nodes
    const nodeGeom = new THREE.SphereGeometry(0.05, 8, 8);
    const nodeMat = new THREE.MeshPhongMaterial({ color: 0xffffff });
    
    nodes.forEach((pos, i) => {
        const node = new THREE.Mesh(nodeGeom, nodeMat);
        node.position.set(pos[0], pos[1] - minY, 0); // Ground it
        group.add(node);
        nodeMeshes.push(node);
    });

    // Create lines for edges (from triangles)
    const lineMat = new THREE.LineBasicMaterial({ color: 0x000000 });
    const edgePairs = new Set();
    
    triangles.forEach(tri => {
        const addEdge = (a, b) => {
            const key = [a, b].sort().join('-');
            if (!edgePairs.has(key)) {
                edgePairs.add(key);
                const points = [
                    new THREE.Vector3(nodes[a][0], nodes[a][1] - minY, 0),
                    new THREE.Vector3(nodes[b][0], nodes[b][1] - minY, 0)
                ];
                const lineGeom = new THREE.BufferGeometry().setFromPoints(points);
                const line = new THREE.Line(lineGeom, lineMat);
                group.add(line);
                edgeLines.push({ line, a, b });
            }
        };
        addEdge(tri[0], tri[1]);
        addEdge(tri[1], tri[2]);
        addEdge(tri[2], tri[0]);
    });

    group.position.set(10, 1, 0);
    // Scale down and center
    group.scale.set(0.5, 0.5, 0.5);
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
    console.log("Start button clicked");
    if (!gameLogic) {
        console.error("Cannot start: gameLogic is not initialized");
        return;
    }
    try {
        gameLogic.start_game(Date.now());
        console.log("Game started in backend");
        document.getElementById('startScreen').style.display = 'none';
    } catch (e) {
        console.error("Failed to start race in backend:", e);
    }
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

    // Sync from Rust simulation if available
    if (!isPythonBackend && gameLogic.get_sim_node_positions) {
        const nodePositions = gameLogic.get_sim_node_positions();
        if (nodePositions && nodeMeshes.length > 0) {
            nodePositions.forEach((pos, i) => {
                if (nodeMeshes[i]) {
                    nodeMeshes[i].position.set(pos[0], pos[1], 0);
                }
            });

            // Update body mesh and edges
            const mesh = playerMorph.children[0];
            const positions = mesh.geometry.attributes.position.array;
            nodeMeshes.forEach((node, i) => {
                positions[i * 3] = node.position.x;
                positions[i * 3 + 1] = node.position.y;
            });
            mesh.geometry.attributes.position.needsUpdate = true;

            edgeLines.forEach(edge => {
                const points = [
                    nodeMeshes[edge.a].position,
                    nodeMeshes[edge.b].position
                ];
                edge.line.geometry.setFromPoints(points);
            });
        }
    }
    
    // Update both creature positions for tug-of-war
    if (playerMorph1 && playerMorph2) {
        playerMorph1.position.x = gameLogic.get_creature1_x();
        playerMorph1.position.y = gameLogic.get_creature1_y();
        playerMorph1.position.z = gameLogic.get_creature1_z();
        playerMorph1.rotation.y = gameLogic.get_creature1_rotation_y();
        
        playerMorph2.position.x = gameLogic.get_creature2_x();
        playerMorph2.position.y = gameLogic.get_creature2_y();
        playerMorph2.position.z = gameLogic.get_creature2_z();
        playerMorph2.rotation.y = gameLogic.get_creature2_rotation_y();
    }
    
    // Update timer UI
    document.getElementById('timer').textContent = gameLogic.get_current_time().toFixed(2) + 's';
    
    // Check for winner
    if (gameLogic.is_completed()) {
        const winner = gameLogic.get_winner();
        if (winner > 0) {
            document.getElementById('endScreen').style.display = 'flex';
            document.getElementById('finalTime').textContent = gameLogic.get_current_time().toFixed(2) + 's';
            document.getElementById('winnerText').textContent = winner === 1 ? "Player 1 Wins!" : "Player 2 Wins!";
        }
    }
    
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
    updateCreaturePositions();
    updateCreatureAnimations();
    
    // Render the scene
    renderer.render(scene, camera);
}

function updateCreaturePositions() {
    if (!gameLogic) return;
    
    // Update creature positions from Rust game state
    playerMorph1.position.x = gameLogic.get_creature1_x();
    playerMorph1.position.z = 2.0; // Top lane
    
    playerMorph2.position.x = gameLogic.get_creature2_x();
    playerMorph2.position.z = -2.0; // Bottom lane
}

function updateCreatureAnimations() {
    if (!gameLogic) return;
    
    // Get speeds for animation
    const speed1 = gameLogic.get_creature1_speed();
    const speed2 = gameLogic.get_creature2_speed();
    
    // Animate legs based on speed
    animateLegs(playerMorph1, speed1);
    animateLegs(playerMorph2, speed2);
}

function animateLegs(creature, speed) {
    const time = Date.now() * 0.002;
    const intensity = Math.min(speed * 0.1, 1.0);
    
    // Animate legs (assuming legs are children 2 and 3)
    if (creature.children.length >= 4) {
        const leftLeg = creature.children[2];
        const rightLeg = creature.children[3];
        
        // Walking animation
        leftLeg.rotation.x = Math.sin(time) * intensity * 0.5;
        rightLeg.rotation.x = Math.sin(time + Math.PI) * intensity * 0.5;
    }
}

// Start the game when the page loads
if (document.readyState === 'complete') {
    init();
} else {
    window.addEventListener('load', init);
}
