use bevy::prelude::*;
use crate::components::building::{Building, BuildingSpawner, ResourceType};
use crate::components::player::PlayerResources;
use crate::components::unit::Team;
// Import the necessary components
use crate::components::resource::Gatherer;
use crate::entities::unit_types::UnitType;
use crate::units::engineer::Engineer;
use crate::components::unit::{Unit, UnitState};
use std::time::Duration;

pub struct ProductionPlugin;

impl Plugin for ProductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_unit_production,).run_if(in_state(crate::GameState::Gameplay))
        );
        
        info!("Production Plugin initialized");
    }
}

/// System to handle unit production from buildings
fn handle_unit_production(
    time: Res<Time>,
    mut buildings: Query<(Entity, &mut BuildingSpawner, &Building, &Transform, &Team)>,
    mut player_resources: Option<ResMut<PlayerResources>>,
    mut commands: Commands,
) {
    for (_entity, mut spawner, building, transform, team) in buildings.iter_mut() {
        // Only process completed buildings
        if !building.is_completed {
            continue;
        }
        
        // Tick the spawn timer
        spawner.spawn_timer.tick(time.delta());
        
        // Check if it's time to spawn a unit
        if spawner.spawn_timer.just_finished() {
            // Get the position of the building
            let building_pos = transform.translation.truncate();
            
            // Determine spawn position (slightly offset from building)
            let spawn_pos = building_pos + Vec2::new(40.0, 0.0);
            
            // Check the unit type to spawn
            match spawner.unit_type.as_str() {
                "Engineer" => {
                    // Check if player has enough resources (if this is a player building)
                    let can_afford = if *team == Team::Player {
                        if let Some(ref mut resources) = player_resources {
                            // Engineer costs 20 Wood and 10 Stone
                            // Find available resources
                            let mut wood_available = 0;
                            let mut stone_available = 0;
                            
                            for (res_type, amount) in &resources.resources {
                                match res_type {
                                    ResourceType::Wood => wood_available = *amount,
                                    ResourceType::Stone => stone_available = *amount,
                                    _ => {},
                                }
                            }
                            
                            if wood_available >= 20 && stone_available >= 10 {
                                // Deduct resources
                                for (res_type, amount) in &mut resources.resources {
                                    match res_type {
                                        ResourceType::Wood => *amount -= 20,
                                        ResourceType::Stone => *amount -= 10,
                                        _ => {}
                                    }
                                }
                                true
                            } else {
                                false
                            }
                        } else {
                            true // No player resources system yet, so always allow
                        }
                    } else {
                        true // AI teams don't check resources for now
                    };
                    
                    if can_afford {
                        // Spawn the engineer directly here
                        let _engineer_entity = spawn_local_engineer(&mut commands, spawn_pos, *team);
                        info!("Spawned Engineer unit for team {:?}", team);
                    } else {
                        info!("Not enough resources to spawn Engineer");
                    }
                },
                // Add other unit types as needed
                _ => {
                    // Default to engineer for now
                    let _engineer_entity = spawn_local_engineer(&mut commands, spawn_pos, *team);
                    info!("Spawned default Engineer unit for team {:?}", team);
                }
            }
            
            // Reset the timer for next spawn
            spawner.spawn_timer = Timer::from_seconds(spawner.spawn_time, TimerMode::Repeating);
        }
    }
}

// Local version of spawn_engineer to avoid module export issues
fn spawn_local_engineer(commands: &mut Commands, position: Vec2, team: Team) -> Entity {
    let color = match team {
        Team::Player => Color::srgb(0.2, 0.6, 0.8), // Blue for player
        Team::Enemy => Color::srgb(0.8, 0.2, 0.2),  // Red for enemy
        Team::Neutral => Color::srgb(0.5, 0.5, 0.5), // Gray for neutral
    };
    
    let engineer = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(24.0, 24.0)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 10.0),
            ..default()
        },
        Unit {
            health: 50.0,
            max_health: 50.0,
            movement_speed: 60.0,
            team,
            attack_power: 5.0,
            attack_range: 1.0,
        },
        UnitType::Engineer,
        Engineer {
            build_speed: 10.0,
            build_timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating),
            target_building: None,
        },
        Gatherer {
            gather_rate: 5.0,
            gather_timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating),
            carry_capacity: 20,
            current_load: 0,
            target_resource: None,
        },
        UnitState::Idle,
        Name::new(format!("{:?} Engineer", team)),
    )).id();
    
    engineer
}
