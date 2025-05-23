use bevy::prelude::*;
use bevy::reflect::Reflect;
use crate::components::unit::Team;
use crate::components::base_modules::ResourceType;

/// The main mobile base that players control
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MechanicalBase {
    pub health: f32,
    pub max_health: f32,
    pub base_movement_speed: f32,  // Base speed without modules
    pub effective_movement_speed: f32,  // Speed after module modifiers
    pub team: Team,
    pub resources: Vec<(ResourceType, i32)>,
    pub power_output: f32,  // Total power generated
    pub power_consumed: f32, // Power currently in use
    pub max_power: f32,      // Maximum power capacity
    pub attachment_points: Vec<Entity>, // References to attachment points
    pub modules: Vec<Entity>, // References to attached modules
}

impl Default for MechanicalBase {
    fn default() -> Self {
        Self {
            health: 1000.0,
            max_health: 1000.0,
            base_movement_speed: 30.0,  // Base speed without modules
            effective_movement_speed: 30.0,  // Will be updated by module system
            team: Team::Player,
            resources: vec![
                (ResourceType::Wood, 100),
                (ResourceType::Stone, 50),
                (ResourceType::Iron, 25),
            ],
            power_output: 100.0,  // Base power generation
            power_consumed: 0.0,  // Starts with no power consumption
            max_power: 150.0,     // Base power capacity
            attachment_points: Vec::new(),
            modules: Vec::new(),
        }
    }
}

impl MechanicalBase {
    /// Calculate total power balance (positive = surplus, negative = deficit)
    pub fn power_balance(&self) -> f32 {
        self.power_output - self.power_consumed
    }
    
    /// Check if the base has enough power to activate a module
    pub fn can_activate_module(&self, power_required: f32) -> bool {
        self.power_balance() + self.power_consumed >= power_required
    }
    
    /// Add a new attachment point
    pub fn add_attachment_point(&mut self, point_entity: Entity) {
        self.attachment_points.push(point_entity);
    }
    
    /// Add a new module
    pub fn add_module(&mut self, module_entity: Entity) {
        self.modules.push(module_entity);
    }
}

/// Player resources and stats
#[derive(Resource)]
pub struct PlayerResources {
    pub resources: Vec<(ResourceType, i32)>,
    pub score: i32,
    pub strategic_points_controlled: i32,
}

impl Default for PlayerResources {
    fn default() -> Self {
        Self {
            resources: vec![
                (ResourceType::Wood, 100),
                (ResourceType::Stone, 50),
                (ResourceType::Iron, 25),
            ],
            score: 0,
            strategic_points_controlled: 0,
        }
    }
}

/// Component for player-controlled entities
#[derive(Component)]
pub struct PlayerControlled;

/// Target location that the base needs to reach
#[derive(Component)]
pub struct StrategicTarget {
    pub position: Vec2,
    pub is_reached: bool,
    pub time_held: f32,
}
