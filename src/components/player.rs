use bevy::prelude::*;
use crate::components::unit::Team;
use crate::components::building::ResourceType;

/// The main mechanical base that players control
#[derive(Component)]
pub struct MechanicalBase {
    pub health: f32,
    pub max_health: f32,
    pub movement_speed: f32,
    pub team: Team,
    pub resources: Vec<(ResourceType, i32)>,
}

impl Default for MechanicalBase {
    fn default() -> Self {
        Self {
            health: 1000.0,
            max_health: 1000.0,
            movement_speed: 50.0, // Base movement speed
            team: Team::Player,
            resources: vec![
                (ResourceType::Wood, 100),
                (ResourceType::Stone, 50),
                (ResourceType::Iron, 25),
            ],
        }
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
