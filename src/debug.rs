use bevy::prelude::*;
use crate::components::player::MechanicalBase;
use crate::components::unit::{Unit, Selected};
use crate::states::game_state::GameState;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            debug_click_entities,
            debug_world_entities,
            debug_key_input,
        ).run_if(in_state(GameState::Gameplay)));
        
        info!("Debug plugin initialized");
    }
}

// Debug system triggered by Spacebar to print all game entities
fn debug_world_entities(
    keyboard: Res<ButtonInput<KeyCode>>,
    bases: Query<(Entity, &Transform, Option<&Selected>, &MechanicalBase)>,
    units: Query<(Entity, &Transform, Option<&Selected>, &Unit)>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        info!("DEBUG: ======= WORLD ENTITY REPORT =========");
        
        // Report on mechanical bases
        if bases.is_empty() {
            info!("DEBUG: NO MECHANICAL BASES FOUND!");
        } else {
            info!("DEBUG: Found {} mechanical bases:", bases.iter().count());
            for (entity, transform, selected, base) in bases.iter() {
                let position = transform.translation;
                let is_selected = selected.is_some();
                info!("DEBUG: Base Entity {:?} at position ({:.1}, {:.1}, {:.1}), team: {:?}, selected: {}", 
                      entity, position.x, position.y, position.z, base.team, is_selected);
            }
        }
        
        // Report on units
        info!("DEBUG: Found {} units:", units.iter().count());
        for (entity, transform, selected, unit) in units.iter() {
            let position = transform.translation;
            let is_selected = selected.is_some();
            info!("DEBUG: Unit Entity {:?} at position ({:.1}, {:.1}, {:.1}), team: {:?}, selected: {}", 
                  entity, position.x, position.y, position.z, unit.team, is_selected);
        }
        
        info!("DEBUG: ======= END REPORT =========");
    }
}

// Debug system to manually select a base using F1 key
fn debug_key_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    bases: Query<(Entity, &MechanicalBase), With<MechanicalBase>>,
    units: Query<Entity, With<Selected>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        info!("DEBUG: Manually selecting first mechanical base");
        
        // Clear existing selections
        for entity in units.iter() {
            commands.entity(entity).remove::<Selected>();
        }
        
        // Select the first base we find
        if let Some((entity, base)) = bases.iter().next() {
            // Insert Selected component
            commands.entity(entity).insert(Selected);
            
            // Instead of trying to modify the sprite directly here,
            // we'll add a component that will be used by another system to change the sprite color
            info!("DEBUG: Selected base entity {:?}, team: {:?}", entity, base.team);
        } else {
            info!("DEBUG: No mechanical bases found to select!");
        }
    }
}

// Debug system to print information about what's under the cursor
fn debug_click_entities(
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    bases: Query<(Entity, &Transform, &Sprite, &MechanicalBase, Option<&Selected>)>,
    _units: Query<(Entity, &Transform, &Sprite, &Unit, Option<&Selected>)>,
) {
    // Only process when right mouse button is pressed (to avoid interference with normal gameplay)
    if mouse_buttons.just_pressed(MouseButton::Right) {
        // Get the primary window
        let window = windows.single();
        
        // Get the camera transform
        let (camera, camera_transform) = camera_q.single();
        
        if let Some(cursor_position) = window.cursor_position() {
            if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                info!("DEBUG: Right-click at screen position: {:?}, world position: {:?}", cursor_position, world_position);
                
                // Check for mechanical bases near the cursor
                let mut found_bases = false;
                for (entity, transform, sprite, base, selected) in bases.iter() {
                    found_bases = true;
                    let base_pos = transform.translation.truncate();
                    let distance = world_position.distance(base_pos);
                    let is_selected = selected.is_some();
                    
                    info!("DEBUG: Found base at {:?}, distance from click: {}, selected: {}, team: {:?}", 
                          base_pos, distance, is_selected, base.team);
                    
                    if let Some(size) = sprite.custom_size {
                        info!("DEBUG: Base size: {:?}, transform scale: {:?}", size, transform.scale);
                        
                        let half_width = size.x / 2.0;
                        let half_height = size.y / 2.0;
                        
                        // Check if click is within bounds
                        if world_position.x >= base_pos.x - half_width && 
                           world_position.x <= base_pos.x + half_width && 
                           world_position.y >= base_pos.y - half_height && 
                           world_position.y <= base_pos.y + half_height {
                            info!("DEBUG: Click is WITHIN BASE BOUNDS! Entity: {:?}", entity);
                        } else {
                            info!("DEBUG: Click is outside base bounds. Entity: {:?}", entity);
                        }
                    }
                }
                
                if !found_bases {
                    info!("DEBUG: NO BASES FOUND IN THE GAME WORLD!");
                }
            }
        }
    }
}
