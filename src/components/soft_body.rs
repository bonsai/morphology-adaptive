use rapier2d::prelude::*;
use serde_json::Value;

pub struct SoftBodySimulation {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub ccd_solver: CCDSolver,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub node_handles: Vec<RigidBodyHandle>,
    pub muscle_joint_handles: Vec<ImpulseJointHandle>,
}

impl SoftBodySimulation {
    pub fn new(mesh_json: &str) -> Self {
        let data: Value = serde_json::from_str(mesh_json).unwrap();
        let nodes = data["pos"].as_array().unwrap();
        let triangles = data["triangles"].as_array().unwrap();

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let mut node_handles = Vec::new();
        let mut muscle_joint_handles = Vec::new();

        // 1. Create nodes as small rigid bodies
        for pos in nodes {
            let x = pos[0].as_f64().unwrap() as f32;
            let y = pos[1].as_f64().unwrap() as f32;

            let rb = RigidBodyBuilder::dynamic()
                .translation(vector![x, y])
                .linear_damping(0.5)
                .angular_damping(0.5)
                .build();
            let handle = rigid_body_set.insert(rb);
            
            let collider = ColliderBuilder::ball(0.05)
                .friction(0.8)
                .restitution(0.2)
                .build();
            collider_set.insert_with_parent(collider, handle, &mut rigid_body_set);
            
            node_handles.push(handle);
        }

        // 2. Create ground
        let ground_rb = RigidBodyBuilder::fixed().translation(vector![0.0, -1.0]).build();
        let ground_handle = rigid_body_set.insert(ground_rb);
        let ground_collider = ColliderBuilder::cuboid(100.0, 1.0).build();
        collider_set.insert_with_parent(ground_collider, ground_handle, &mut rigid_body_set);

        // 3. Create muscles (spring joints) between nodes based on triangles
        let mut edges = std::collections::HashSet::new();
        for tri in triangles {
            let tri_indices = tri.as_array().unwrap();
            let idxs = [
                tri_indices[0].as_u64().unwrap() as usize,
                tri_indices[1].as_u64().unwrap() as usize,
                tri_indices[2].as_u64().unwrap() as usize,
            ];

            let pairs = [(idxs[0], idxs[1]), (idxs[1], idxs[2]), (idxs[2], idxs[0])];
            for (a, b) in pairs {
                let key = if a < b { (a, b) } else { (b, a) };
                if !edges.contains(&key) {
                    edges.insert(key);
                    
                    let pos_a = nodes[a].as_array().unwrap();
                    let pos_b = nodes[b].as_array().unwrap();
                    let dist = ((pos_a[0].as_f64().unwrap() - pos_b[0].as_f64().unwrap()).powi(2) + 
                               (pos_a[1].as_f64().unwrap() - pos_b[1].as_f64().unwrap()).powi(2)).sqrt() as f32;

                    // Use a spring-like joint
                    let joint = SpringJointBuilder::new(dist, 1000.0, 10.0)
                        .local_anchor1(point![0.0, 0.0])
                        .local_anchor2(point![0.0, 0.0])
                        .build();
                    
                    let handle = impulse_joint_set.insert(node_handles[a], node_handles[b], joint, true);
                    muscle_joint_handles.push(handle);
                }
            }
        }

        Self {
            rigid_body_set,
            collider_set,
            impulse_joint_set,
            multibody_joint_set,
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            ccd_solver: CCDSolver::new(),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            node_handles,
            muscle_joint_handles,
        }
    }

    pub fn step(&mut self, dt: f32, muscle_activations: &[f32]) {
        self.integration_parameters.dt = dt;
        let gravity = vector![0.0, -9.81];

        // Apply muscle activations by changing spring rest length
        for (i, &activation) in muscle_activations.iter().enumerate() {
            if i < self.muscle_joint_handles.len() {
                let handle = self.muscle_joint_handles[i];
                if let Some(joint) = self.impulse_joint_set.get_mut(handle) {
                    // Joint data is a private field, so we use specific builder or accessors
                    // For now, we skip detailed modification until the basic structure is stable
                }
            }
        }
        
        self.physics_pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &mut (),
            &mut (),
        );
    }

    pub fn get_node_positions(&self) -> Vec<[f32; 2]> {
        self.node_handles.iter().map(|&h| {
            let rb = &self.rigid_body_set[h];
            [rb.translation().x, rb.translation().y]
        }).collect()
    }
}
