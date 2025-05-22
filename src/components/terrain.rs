use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TerrainType {
    Plains,      // Normal movement speed
    Forest,      // Slows movement, provides wood resources
    Mountain,    // Very slow movement, provides stone resources
    Water,       // Impassable for land units
    MetalDeposit, // Provides iron resources
    RoughTerrain, // Slows movement
    StrategicLocation, // Target locations to capture and hold
}

#[derive(Component)]
pub struct Terrain {
    pub terrain_type: TerrainType,
    pub movement_modifier: f32, // Multiplier for movement speed (1.0 = normal, <1.0 = slower)
    pub visibility_modifier: f32, // Affects visibility range (1.0 = normal, <1.0 = reduced)
    pub defense_modifier: f32, // Defensive bonus (1.0 = normal, >1.0 = better defense)
}

#[derive(Component)]
pub struct MapTile {
    pub grid_x: i32,
    pub grid_y: i32,
    pub is_explored: bool,
    pub is_visible: bool,
}

#[derive(Component)]
pub struct StrategicPoint {
    pub capture_progress: f32, // 0.0 to 1.0
    pub controlling_team: Option<crate::components::unit::Team>,
    pub capture_value: i32, // Points value or strategic importance
    pub position: Vec2, // World position of the strategic point
}

impl Terrain {
    pub fn new(terrain_type: TerrainType) -> Self {
        match terrain_type {
            TerrainType::Plains => Self {
                terrain_type,
                movement_modifier: 1.0,
                visibility_modifier: 1.0,
                defense_modifier: 1.0,
            },
            TerrainType::Forest => Self {
                terrain_type,
                movement_modifier: 0.7,
                visibility_modifier: 0.6,
                defense_modifier: 1.3,
            },
            TerrainType::Mountain => Self {
                terrain_type,
                movement_modifier: 0.4,
                visibility_modifier: 1.5, // Better visibility from high ground
                defense_modifier: 1.5,
            },
            TerrainType::Water => Self {
                terrain_type,
                movement_modifier: 0.0, // Impassable
                visibility_modifier: 1.0,
                defense_modifier: 0.0, // No defense bonus (can't stop on water)
            },
            TerrainType::MetalDeposit => Self {
                terrain_type,
                movement_modifier: 0.8,
                visibility_modifier: 1.0,
                defense_modifier: 1.1,
            },
            TerrainType::RoughTerrain => Self {
                terrain_type,
                movement_modifier: 0.6,
                visibility_modifier: 0.8,
                defense_modifier: 1.2,
            },
            TerrainType::StrategicLocation => Self {
                terrain_type,
                movement_modifier: 1.0,
                visibility_modifier: 1.2,
                defense_modifier: 1.3,
            },
        }
    }
}
