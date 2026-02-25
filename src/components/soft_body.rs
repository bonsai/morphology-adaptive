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
}

impl SoftBodySimulation {
    pub fn new(mesh_json: &str) -> Self {
        let data: Value = serde_json::from_str(mesh_json).unwrap();
        let nodes = data["pos"].as_array().unwrap();

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let mut node_handles = Vec::new();

        // Create nodes
        for pos in nodes {
            let x = pos[0].as_f64().unwrap() as f32;
            let y = pos[1].as_f64().unwrap() as f32;

            let rb = RigidBodyBuilder::dynamic()
                .translation(vector![x, y])
                .build();
            let handle = rigid_body_set.insert(rb);
            
            let collider = ColliderBuilder::ball(0.05).build();
            collider_set.insert_with_parent(collider, handle, &mut rigid_body_set);
            
            node_handles.push(handle);
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
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.integration_parameters.dt = dt;
        let gravity = vector![0.0, -9.81];
        
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
}
