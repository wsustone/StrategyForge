use bevy::prelude::*;
use crate::components::player::MechanicalBase;
use crate::components::unit::{UnitState, Selected};
use crate::GameState;

/// Component for a target location the mechanical base should move to
#[derive(Component)]
pub struct MoveTarget {
    pub target_position: Vec2,
}

/// Plugin to handle mechanical base movement
pub struct BaseMovePlugin;

impl Plugin for BaseMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_base_movement,
                set_base_move_target,
            ).run_if(in_state(GameState::Gameplay))
        );
        
        info!("Base Movement Plugin initialized");
    }
}

/// System to handle setting a move target for selected bases when right-clicking
fn set_base_move_target(
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    selected_bases: Query<Entity, (With<Selected>, With<MechanicalBase>)>,
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
                // Set move target for all selected bases
                for entity in selected_bases.iter() {
                    // Remove any existing move target
                    commands.entity(entity).remove::<MoveTarget>();
                    
                    // Add new move target
                    commands.entity(entity).insert(MoveTarget {
                        target_position: world_position,
                    });
                    
                    // Set unit state to Moving
                    commands.entity(entity).insert(UnitState::Moving);
                    
                    info!("Setting base move target to: {:?}", world_position);
                }
            }
        }
    }
}

/// System to move mechanical bases toward their target positions
fn handle_base_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &MechanicalBase, &MoveTarget, &UnitState)>,
) {
    for (entity, mut transform, base, move_target, state) in query.iter_mut() {
        // Only move if the unit state is Moving
        if matches!(state, UnitState::Moving) {
            let current_position = transform.translation.truncate();
            let target_position = move_target.target_position;
            
            // Calculate direction vector
            let direction = (target_position - current_position).normalize_or_zero();
            
            // Calculate movement distance this frame
            let movement_speed = base.movement_speed;
            let movement_distance = movement_speed * time.delta_seconds();
            
            // Calculate new position
            let new_position = current_position + direction * movement_distance;
            
            // Update transform
            transform.translation.x = new_position.x;
            transform.translation.y = new_position.y;
            
            // Check if we've reached the target (within a small threshold)
            let distance_to_target = current_position.distance(target_position);
            if distance_to_target < 5.0 {
                // We've arrived, remove the move target and set state to Idle
                commands.entity(entity).remove::<MoveTarget>();
                commands.entity(entity).insert(UnitState::Idle);
                info!("Base reached its destination");
            }
        }
    }
}
