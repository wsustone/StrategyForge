use bevy::prelude::*;
use crate::components::unit::{Unit, Team, UnitState};

/// Defines the different types of units available in the game
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitType {
    // Gatherer units
    Engineer,
    Gatherer,
    
    // Combat units - Land
    LandToLandTank,
    LandToAirTank,
    Artillery,
    
    // Combat units - Air
    AirToAirFighter,
    AirToLandBomber,
    
    // Special large units
    LargeTank,
    LargeHoveringAircraft,
    LargeBomber,
    LargeArtillery,
}

impl UnitType {
    pub fn spawn_unit(&self, commands: &mut Commands, position: Vec2, team: Team) -> Entity {
        let (health, attack_power, attack_range, movement_speed, _color) = match self {
            // Gatherer units
            UnitType::Engineer => (50.0, 5.0, 1.0, 60.0, Color::srgb(0.2, 0.8, 0.2)),
            UnitType::Gatherer => (40.0, 2.0, 1.0, 70.0, Color::srgb(0.2, 0.7, 0.3)),
            
            // Combat units - Land
            UnitType::LandToLandTank => (100.0, 15.0, 5.0, 40.0, Color::srgb(0.5, 0.5, 0.5)),
            UnitType::LandToAirTank => (90.0, 12.0, 8.0, 45.0, Color::srgb(0.6, 0.6, 0.6)),
            UnitType::Artillery => (70.0, 25.0, 12.0, 25.0, Color::srgb(0.7, 0.7, 0.7)),
            
            // Combat units - Air
            UnitType::AirToAirFighter => (60.0, 18.0, 6.0, 90.0, Color::srgb(0.3, 0.3, 0.8)),
            UnitType::AirToLandBomber => (80.0, 30.0, 5.0, 70.0, Color::srgb(0.4, 0.4, 0.8)),
            
            // Special large units
            UnitType::LargeTank => (200.0, 30.0, 7.0, 25.0, Color::srgb(0.8, 0.8, 0.2)),
            UnitType::LargeHoveringAircraft => (150.0, 25.0, 10.0, 50.0, Color::srgb(0.2, 0.8, 0.8)),
            UnitType::LargeBomber => (120.0, 40.0, 8.0, 60.0, Color::srgb(0.8, 0.2, 0.8)),
            UnitType::LargeArtillery => (150.0, 50.0, 15.0, 20.0, Color::srgb(0.9, 0.6, 0.1)),
        };
        
        let _team_color = match team {
            Team::Player => Color::srgb(0.2, 0.6, 0.8),
            Team::Enemy => Color::srgb(0.8, 0.2, 0.2),
            Team::Neutral => Color::srgb(0.7, 0.7, 0.7),
        };
        
        // Combine unit type color with team color
        // For Bevy 0.14.0, let's use a simpler approach that just mixes the team color with the unit color
        // We'll just create a slightly different color based on the unit type and team
        let final_color = if team == Team::Player {
            Color::srgb(0.2, 0.6, 0.8) // Blue for player units
        } else if team == Team::Enemy {
            Color::srgb(0.8, 0.2, 0.2) // Red for enemy units
        } else {
            Color::srgb(0.7, 0.7, 0.7) // Gray for neutral units
        };
        
        // Create the unit entity
        let entity = commands.spawn_empty().id();
        
        // Add components one by one
        commands.entity(entity)
            .insert(SpriteBundle {
                sprite: Sprite {
                    color: final_color,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, 5.0),
                ..default()
            })
            .insert(Unit {
                health,
                max_health: health,
                attack_power,
                attack_range,
                movement_speed,
                team,
            })
            .insert(UnitState::Idle)
            .insert(*self);
            
        entity
    }
}
