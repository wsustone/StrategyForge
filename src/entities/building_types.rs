use bevy::prelude::*;
use crate::components::building::{Building, BuildingSpawner, ResourceGenerator, Constructable, ResourceType};
use crate::components::unit::Team;

/// Defines the different types of buildings available in the game
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildingType {
    // Resource production
    Sawmill,       // Produces Wood
    StoneMine,     // Produces Stone
    IronMine,      // Produces Iron
    
    // Unit production
    Barracks,      // Produces combat units
    Workshop,      // Produces vehicles and artillery
    Airfield,      // Produces air units
    
    // Defense
    Turret,        // Basic defense
    AntiAirTurret, // Air defense
    
    // Special
    CommandCenter, // Main building, required for tech upgrades
    ResearchLab,   // Unlocks upgrades
}

impl BuildingType {
    pub fn spawn_building(&self, commands: &mut Commands, position: Vec2, team: Team) -> Entity {
        let (health, construction_time, _color, size) = match self {
            // Resource production
            BuildingType::Sawmill => (200.0, 15.0, Color::srgb(0.6, 0.4, 0.2), Vec2::new(24.0, 24.0)),
            BuildingType::StoneMine => (250.0, 20.0, Color::srgb(0.5, 0.5, 0.5), Vec2::new(24.0, 24.0)),
            BuildingType::IronMine => (250.0, 25.0, Color::srgb(0.6, 0.6, 0.7), Vec2::new(24.0, 24.0)),
            
            // Unit production
            BuildingType::Barracks => (300.0, 30.0, Color::srgb(0.3, 0.3, 0.6), Vec2::new(32.0, 32.0)),
            BuildingType::Workshop => (350.0, 40.0, Color::srgb(0.5, 0.3, 0.3), Vec2::new(40.0, 40.0)),
            BuildingType::Airfield => (250.0, 35.0, Color::srgb(0.3, 0.5, 0.6), Vec2::new(48.0, 48.0)),
            
            // Defense
            BuildingType::Turret => (200.0, 15.0, Color::srgb(0.7, 0.3, 0.3), Vec2::new(16.0, 16.0)),
            BuildingType::AntiAirTurret => (180.0, 20.0, Color::srgb(0.3, 0.7, 0.3), Vec2::new(16.0, 16.0)),
            
            // Special
            BuildingType::CommandCenter => (500.0, 60.0, Color::srgb(0.8, 0.8, 0.3), Vec2::new(48.0, 48.0)),
            BuildingType::ResearchLab => (200.0, 45.0, Color::srgb(0.3, 0.3, 0.8), Vec2::new(32.0, 32.0)),
        };
        
        // Resource costs based on building type
        let resource_cost = match self {
            BuildingType::Sawmill => vec![(ResourceType::Wood, 50), (ResourceType::Stone, 30)],
            BuildingType::StoneMine => vec![(ResourceType::Wood, 60), (ResourceType::Stone, 20)],
            BuildingType::IronMine => vec![(ResourceType::Wood, 60), (ResourceType::Stone, 40)],
            
            BuildingType::Barracks => vec![(ResourceType::Wood, 80), (ResourceType::Stone, 50)],
            BuildingType::Workshop => vec![(ResourceType::Wood, 100), (ResourceType::Stone, 80), (ResourceType::Iron, 30)],
            BuildingType::Airfield => vec![(ResourceType::Wood, 120), (ResourceType::Stone, 60), (ResourceType::Iron, 40)],
            
            BuildingType::Turret => vec![(ResourceType::Wood, 30), (ResourceType::Stone, 40), (ResourceType::Iron, 20)],
            BuildingType::AntiAirTurret => vec![(ResourceType::Wood, 30), (ResourceType::Stone, 30), (ResourceType::Iron, 40)],
            
            BuildingType::CommandCenter => vec![(ResourceType::Wood, 200), (ResourceType::Stone, 150), (ResourceType::Iron, 100)],
            BuildingType::ResearchLab => vec![(ResourceType::Wood, 120), (ResourceType::Stone, 80), (ResourceType::Iron, 80)],
        };
        
        // Adjust color based on team
        let _team_color = match team {
            Team::Player => Color::srgb(0.2, 0.6, 0.8),
            Team::Enemy => Color::srgb(0.8, 0.2, 0.2),
            Team::Neutral => Color::srgb(0.7, 0.7, 0.7),
        };
        
        // Combine building type color with team color
        // For Bevy 0.14.0, let's use a simpler approach that sets color based on building and team type
        // We'll just create a color based on the building type and team
        let final_color = if team == Team::Player {
            // Adjust player building colors based on type
            match self {
                BuildingType::CommandCenter => Color::srgb(0.3, 0.6, 0.9),
                _ => Color::srgb(0.2, 0.6, 0.8) // Blue for player buildings
            }
        } else if team == Team::Enemy {
            Color::srgb(0.8, 0.2, 0.2) // Red for enemy buildings
        } else {
            Color::srgb(0.7, 0.7, 0.7) // Gray for neutral buildings
        };
        
        // Create components based on building type
        let entity = commands.spawn_empty().id();
        
        // Add basic components first
        commands.entity(entity)
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: final_color,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, 2.0),
                ..default()
            })
            .insert(Building {
                health,
                max_health: health,
                construction_progress: 0.0,
                is_completed: false,
            })
            .insert(Constructable {
                construction_time,
                resource_cost,
            })
            .insert(*self)
            .insert(team);
        
        // Add specialized components based on building type
        match self {
            BuildingType::Sawmill => {
                commands.entity(entity).insert(ResourceGenerator {
                    resource_type: ResourceType::Wood,
                    generation_rate: 5.0,
                    generation_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                });
            },
            BuildingType::StoneMine => {
                commands.entity(entity).insert(ResourceGenerator {
                    resource_type: ResourceType::Stone,
                    generation_rate: 3.0,
                    generation_timer: Timer::from_seconds(4.0, TimerMode::Repeating),
                });
            },
            BuildingType::IronMine => {
                commands.entity(entity).insert(ResourceGenerator {
                    resource_type: ResourceType::Iron,
                    generation_rate: 2.0,
                    generation_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
                });
            },
            BuildingType::Barracks | BuildingType::Workshop | BuildingType::Airfield => {
                commands.entity(entity).insert(BuildingSpawner {
                    unit_type: "Generic".to_string(), // Will be set when spawning specific units
                    spawn_time: 10.0,
                    spawn_timer: Timer::from_seconds(10.0, TimerMode::Repeating),
                });
            },
            _ => {},
        }
        
        // Return the entity ID directly
        entity
    }
}
