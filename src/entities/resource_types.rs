use bevy::prelude::*;
use crate::components::resource::ResourceNode;
use crate::components::building::ResourceType;

/// Defines the different types of resource nodes in the game
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceNodeType {
    WoodSource,    // Forest
    StoneDeposit,  // Mountain
    IronDeposit,   // Metal deposit
}

impl ResourceNodeType {
    pub fn spawn_resource_node(&self, commands: &mut Commands, position: Vec2) -> Entity {
        let (resource_type, amount, max_gatherers, color, size) = match self {
            ResourceNodeType::WoodSource => (
                ResourceType::Wood, 
                1000, 
                3, 
                Color::srgb(0.0, 0.5, 0.0), 
                Vec2::new(24.0, 24.0)
            ),
            ResourceNodeType::StoneDeposit => (
                ResourceType::Stone, 
                800, 
                2, 
                Color::srgb(0.5, 0.5, 0.5), 
                Vec2::new(24.0, 24.0)
            ),
            ResourceNodeType::IronDeposit => (
                ResourceType::Iron, 
                600, 
                2, 
                Color::srgb(0.6, 0.6, 0.7), 
                Vec2::new(24.0, 24.0)
            ),
        };
        
        // Create resource node entity
        let entity = commands.spawn_empty().id();
        
        // Add components one by one
        commands.entity(entity)
            .insert(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, 1.0),
                ..default()
            })
            .insert(ResourceNode {
                resource_type,
                amount_remaining: amount,
                max_gatherers,
                current_gatherers: 0,
            })
            .insert(*self);
            
        entity
    }
    
    /// Get the appropriate resource node type for a terrain type
    pub fn from_terrain(terrain_type: &crate::components::terrain::TerrainType) -> Option<Self> {
        match terrain_type {
            crate::components::terrain::TerrainType::Forest => Some(ResourceNodeType::WoodSource),
            crate::components::terrain::TerrainType::Mountain => Some(ResourceNodeType::StoneDeposit),
            crate::components::terrain::TerrainType::MetalDeposit => Some(ResourceNodeType::IronDeposit),
            _ => None,
        }
    }
}
