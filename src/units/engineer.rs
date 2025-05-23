use bevy::prelude::*;
use bevy::ecs::system::ParamSet;
use crate::components::unit::{Unit, Team, UnitState};
use crate::components::resource::{Gatherer, ResourceNode};
use crate::components::building::{Building, ResourceType, Constructable};
use crate::entities::unit_types::UnitType;
use std::time::Duration;

// Plugin for Engineer unit functionality
pub struct EngineerPlugin;



impl Plugin for EngineerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    handle_engineer_selection,
                    handle_engineer_resource_gathering,
                ).run_if(in_state(crate::GameState::Gameplay))
            );
            
        // Temporarily remove handle_engineer_building from systems until fully fixed
            
        info!("Engineer Plugin initialized");
    }
}

// Component to mark a unit as an Engineer
#[derive(Component)]
pub struct Engineer {
    pub build_speed: f32,
    pub build_timer: Timer,
    pub target_building: Option<Entity>,
}

// Component for the currently selected resource node
#[derive(Component)]
pub struct SelectedResource {
    pub resource_entity: Entity,
}

// Component for the currently selected build location
#[derive(Component)]
pub struct BuildLocation {
    pub position: Vec2,
    pub building_type: String,
}

// System to handle engineer selection and right-click commands
pub fn handle_engineer_selection(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    resource_nodes: Query<(Entity, &Transform, &ResourceNode)>,
    engineers: Query<(Entity, &Transform), (With<Engineer>, With<crate::components::unit::Selected>)>,
    mut commands: Commands,
) {
    // Only process when right mouse button is just pressed
    if mouse_buttons.just_pressed(MouseButton::Right) {
        // Get the primary window
        let window = windows.single();
        
        // Get the camera transform
        let (camera, camera_transform) = camera_q.single();
        
        if let Some(cursor_position) = window.cursor_position() {
            // Convert screen position to world position
            if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                // Check if any selected engineers
                let selected_engineers: Vec<_> = engineers.iter().collect();
                
                if !selected_engineers.is_empty() {
                    // Check if clicked on a resource node
                    for (resource_entity, resource_transform, _) in resource_nodes.iter() {
                        let resource_pos = resource_transform.translation.truncate();
                        let distance = world_position.distance(resource_pos);
                        
                        // If clicked close enough to a resource node
                        if distance < 50.0 {
                            info!("Engineer assigned to gather resource");
                            
                            // Assign all selected engineers to gather from this resource
                            for (engineer_entity, _) in selected_engineers.iter() {
                                // Remove any existing gathering target
                                commands.entity(*engineer_entity).remove::<SelectedResource>();
                                
                                // Assign new gathering target
                                commands.entity(*engineer_entity).insert(SelectedResource {
                                    resource_entity,
                                });
                                
                                // Set state to gathering
                                commands.entity(*engineer_entity).insert(UnitState::Gathering);
                            }
                            
                            return;
                        }
                    }
                    
                    // If not clicked on a resource, set as build location (simplified for now)
                    for (engineer_entity, _) in selected_engineers.iter() {
                        // Remove any existing build location
                        commands.entity(*engineer_entity).remove::<BuildLocation>();
                        
                        // Set new build location (for simplicity, always build a basic structure)
                        commands.entity(*engineer_entity).insert(BuildLocation {
                            position: world_position,
                            building_type: "Barracks".to_string(), // Default building type for now
                        });
                        
                        // Set state to building
                        commands.entity(*engineer_entity).insert(UnitState::Building);
                    }
                }
            }
        }
    }
}

// System to handle resource gathering by engineers
pub fn handle_engineer_resource_gathering(
    time: Res<Time>,
    mut query_set: ParamSet<(
        Query<(Entity, &mut Gatherer, &SelectedResource, &mut Transform), With<Engineer>>,
        Query<(Entity, &mut ResourceNode, &Transform)>
    )>,
) {
    // First get the engineers and store their data
    let mut engineer_data = Vec::new();
    for (entity, _gatherer, selected_resource, transform) in query_set.p0().iter() {
        engineer_data.push((entity, selected_resource.resource_entity, transform.translation.truncate()));
    }
    
    // Then process the engineers
    for (engineer_entity, resource_entity, engineer_pos) in engineer_data.iter() {
        // Try to get the resource node
        if let Ok((_entity, _resource_node, resource_transform)) = query_set.p1().get(*resource_entity) {
            // Get resource position from transform
            let resource_pos = resource_transform.translation.truncate();
            let distance = engineer_pos.distance(resource_pos);
            
            if distance > 30.0 {
                // Engineer is too far, need to move closer
                // Calculate direction to the resource
                let direction = (resource_pos - *engineer_pos).normalize();
                
                // Move toward the resource
                let new_pos = *engineer_pos + direction * 60.0 * time.delta_seconds();
                
                // Update engineer position in a separate step
                let mut engineers = query_set.p0();
                if let Ok((_entity, _gatherer, _selected_resource, mut transform)) = engineers.get_mut(*engineer_entity) {
                    transform.translation.x = new_pos.x;
                    transform.translation.y = new_pos.y;
                }
            } else {
                // Engineer is close enough to gather
                // We need to handle mutable borrows carefully to avoid borrowing query_set multiple times
                
                // Step 1: Check if we can gather (tick the timer)
                let mut should_gather = false;
                let mut gather_rate = 0.0;
                let mut carry_capacity = 0;
                let mut current_load = 0;
                {
                    let mut engineers = query_set.p0();
                    if let Ok((_entity, mut gatherer, _selected_resource, _transform)) = engineers.get_mut(*engineer_entity) {
                        gatherer.gather_timer.tick(time.delta());
                        
                        if gatherer.gather_timer.just_finished() {
                            should_gather = true;
                            gather_rate = gatherer.gather_rate;
                            carry_capacity = gatherer.carry_capacity;
                            current_load = gatherer.current_load;
                        }
                    }
                }
                
                // Step 2: If we should gather, check the resource node
                let mut resource_type = None;
                let mut amount_gathered = 0;
                let mut updated_load = current_load;
                
                if should_gather {
                    let mut resource_nodes = query_set.p1();
                    if let Ok((_entity, mut resource_node, _transform)) = resource_nodes.get_mut(*resource_entity) {
                        // Gather resources if there are any left
                        if resource_node.amount_remaining > 0 {
                            let gather_amount = (gather_rate * time.delta_seconds()) as i32;
                            let actual_amount = gather_amount.min(resource_node.amount_remaining);
                            
                            // Update resource node
                            resource_node.amount_remaining -= actual_amount;
                            updated_load += actual_amount;
                            
                            resource_type = Some(resource_node.resource_type.clone());
                            amount_gathered = actual_amount;
                        }
                    }
                }
                
                // Step 3: Update the engineer's current load if needed
                if amount_gathered > 0 {
                    let mut engineers = query_set.p0();
                    if let Ok((_entity, mut gatherer, _selected_resource, _transform)) = engineers.get_mut(*engineer_entity) {
                        gatherer.current_load = updated_load;
                        
                        // Check if engineer is full
                        if gatherer.current_load >= carry_capacity {
                            // Would need to find a base to deliver to
                            // For now, just reset the load
                            gatherer.current_load = 0;
                            info!("Engineer delivered resources to base");
                        }
                    }
                }
                
                // Log the gathering outside of the borrow scopes
                if let Some(res_type) = resource_type {
                    info!("Engineer gathered {} units of {:?}", amount_gathered, res_type);
                }
            }
        }
    }


// System to handle building construction by engineers
pub fn handle_engineer_building(
    time: Res<Time>,
    mut engineers: Query<(Entity, &BuildLocation, &mut Engineer, &mut Transform), (With<UnitState>, With<Engineer>)>,
    mut commands: Commands,
) {
    for (_engineer_entity, build_location, mut engineer, mut transform) in engineers.iter_mut() {
        // Move toward the build location if not close enough
        let engineer_pos = transform.translation.truncate();
        let build_pos = build_location.position;
        let distance = engineer_pos.distance(build_pos);
        
        if distance > 30.0 {
            // Calculate direction to the build location
            let direction = (build_pos - engineer_pos).normalize();
            
            // Move toward the build location
            let new_pos = engineer_pos + direction * 60.0 * time.delta_seconds();
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
        } else {
            // Engineer is close enough to build
            // Check if there's already a building in progress
            if let Some(_target_building) = engineer.target_building {
                // Continue building an existing structure
                engineer.build_timer.tick(time.delta());
                
                if engineer.build_timer.just_finished() {
                    info!("Engineer continued construction on building");
                    // Construction progress would be updated on the building entity
                }
            } else {
                // Start building a new structure
                info!("Engineer started construction on a new {:?}", build_location.building_type);
                
                // Spawn a new building entity
                let building_entity = commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgba(0.7, 0.5, 0.3, 1.0),
                            custom_size: Some(Vec2::new(64.0, 64.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(build_pos.x, build_pos.y, 2.0),
                        ..default()
                    },
                    Building {
                        health: 100.0,
                        max_health: 100.0,
                        construction_progress: 0.0,
                        is_completed: false,
                    },
                    Constructable {
                        construction_time: 10.0,
                        resource_cost: vec![(ResourceType::Wood, 50), (ResourceType::Stone, 30)],
                    },
                    Name::new(format!("{}", build_location.building_type)),
                )).id();
                
                // Assign the engineer to this building
                engineer.target_building = Some(building_entity);
                engineer.build_timer = Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating);
            }
        }
    }
}

// Spawn an engineer unit at the given position for the given team
pub fn spawn_engineer(commands: &mut Commands, position: Vec2, team: Team) -> Entity {
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

}
