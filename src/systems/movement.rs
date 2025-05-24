use bevy::prelude::*;
use crate::states::game_state::GameState;
use crate::components::unit::{Unit, UnitState, Selected};

// Simple component to mark a unit's destination
#[derive(Component, Debug)]
pub struct MoveTarget {
    pub position: Vec2,
}

// Movement systems plugin
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    handle_right_click,
                    move_units,
                ).run_if(in_state(GameState::Gameplay)),
            );
        
        info!("Movement plugin initialized");
    }
}

// System to handle right-click for movement commands
pub fn handle_right_click(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    selected_units: Query<Entity, With<Selected>>,
) {
    // Check if right mouse button was just pressed
    if mouse_input.just_pressed(MouseButton::Right) {
        // Get the window and cursor position
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                // Get camera to convert screen coordinates to world coordinates
                if let Ok((camera, camera_transform)) = camera_q.get_single() {
                    // Convert cursor position to world coordinates
                    if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                        let distance = ray.origin.z / ray.direction.z.abs();
                        let world_pos = ray.origin - ray.direction * distance;
                        let target_pos = Vec2::new(world_pos.x, world_pos.y);
                        
                        info!("Right-clicked at world position: {:?}", target_pos);
                        
                        // Set move target for all selected units
                        for entity in selected_units.iter() {
                            commands.entity(entity).insert(MoveTarget {
                                position: target_pos,
                            });
                            
                            // Set unit state to Moving
                            commands.entity(entity).insert(UnitState::Moving);
                        }
                    }
                }
            }
        }
    }
}

// System to move units toward their targets
pub fn move_units(
    mut commands: Commands,
    time: Res<Time>,
    mut units: Query<(Entity, &mut Transform, &Unit, &MoveTarget)>,
) {
    for (entity, mut transform, unit, target) in units.iter_mut() {
        // Get current position and target position
        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let target_pos = target.position;
        
        // Calculate direction to target
        let direction = (target_pos - current_pos).normalize_or_zero();
        if direction == Vec2::ZERO {
            continue; // Skip if we're already at the target or can't determine direction
        }
        
        // Calculate movement for this frame
        let move_speed = unit.movement_speed * time.delta_seconds();
        let move_delta = direction * move_speed;
        
        // Update position
        transform.translation.x += move_delta.x;
        transform.translation.y += move_delta.y;
        
        // Check if we've reached the target (within a small distance)
        let new_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let distance_to_target = new_pos.distance(target_pos);
        
        if distance_to_target < 10.0 {
            // Target reached, remove movement target and set state to Idle
            commands.entity(entity).remove::<MoveTarget>();
            commands.entity(entity).insert(UnitState::Idle);
            info!("Unit reached destination. Distance: {}", distance_to_target);
        }
    }
}






