use bevy::prelude::*;
use crate::GameState;
use crate::utils::font_loader::get_font_handle;
use crate::components::unit::{Unit, Team};
use crate::components::player::MechanicalBase;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), setup_gameplay)
           .add_systems(
                Update,
                (
                    handle_input,
                    unit_selection,
                    update_selection_visuals,
                    update_selection_box_visual, // Add system for selection box visualization
                    handle_game_over_condition,
                ).run_if(in_state(GameState::Gameplay))
           )
           .add_systems(OnExit(GameState::Gameplay), cleanup_gameplay);
    }
}

// Component markers
#[derive(Component)]
struct GameplayUI;

#[derive(Component)]
struct SelectionBoxVisual;

#[derive(Component)]
struct Selected;

// Using Team from unit.rs instead of defining it here

// Resources
#[derive(Resource)]
struct GameResources {
    gold: i32,
    wood: i32,
    stone: i32,
}

#[derive(Resource)]
struct SelectionBox {
    start: Option<Vec2>,
    end: Option<Vec2>,
}

fn setup_gameplay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Game camera (we'll use the one from the CameraPlugin)
    
    // Initialize game resources
    commands.insert_resource(GameResources {
        gold: 500,
        wood: 300,
        stone: 200,
    });
    
    commands.insert_resource(SelectionBox {
        start: None,
        end: None,
    });
    
    // Setup basic UI
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            GameplayUI,
        ))
        .with_children(|parent| {
            // Top resource bar
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    column_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            })
            .with_children(|parent| {
                // Gold
                parent.spawn(TextBundle::from_section(
                    "Gold: 500",
                    TextStyle {
                        font: get_font_handle(&asset_server),
                        font_size: 20.0,
                        color: Color::srgb(1.0, 0.9, 0.0),
                    },
                ));
                
                // Wood
                parent.spawn(TextBundle::from_section(
                    "Wood: 300",
                    TextStyle {
                        font: get_font_handle(&asset_server),
                        font_size: 20.0,
                        color: Color::srgb(0.6, 0.4, 0.2),
                    },
                ));
                
                // Stone
                parent.spawn(TextBundle::from_section(
                    "Stone: 200",
                    TextStyle {
                        font: get_font_handle(&asset_server),
                        font_size: 20.0,
                        color: Color::srgb(0.7, 0.7, 0.7),
                    },
                ));
            });
            
            // Bottom minimap and control panel
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(30.0),
                    height: Val::Px(150.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
                ..default()
            });
        });
    
    // Spawn a few example units
    spawn_example_units(&mut commands);
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Press Escape to go back to main menu
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

fn spawn_example_units(commands: &mut Commands) {
    // Spawn some example units for testing purposes
    // These will be replaced with actual unit spawning systems in the future

    // Spawn player's units
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.2, 0.2, 0.8), // Blue
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-200.0 + i as f32 * 50.0, -200.0, 5.0),
                ..default()
            },
            Unit {
                health: 100.0,
                max_health: 100.0,
                movement_speed: 100.0,
                team: Team::Player,
                attack_power: 10.0,
                attack_range: 50.0,
            },
            // Add unit state as a separate component
            crate::components::unit::UnitState::Idle,
            Name::new(format!("Player Unit {}", i)),
        ));
    }

    // Spawn enemy units
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.8, 0.2, 0.2), // Red
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(200.0 - i as f32 * 50.0, 200.0, 5.0),
                ..default()
            },
            Unit {
                health: 100.0,
                max_health: 100.0,
                movement_speed: 100.0,
                team: Team::Enemy,
                attack_power: 10.0,
                attack_range: 50.0,
            },
            // Add unit state as a separate component
            crate::components::unit::UnitState::Idle,
            Name::new(format!("Enemy Unit {}", i)),
        ));
    }
}

fn unit_selection(
    mut commands: Commands,
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut selection_box: ResMut<SelectionBox>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut units: Query<(Entity, &Transform, &mut Sprite, &Unit)>,
    mut bases: Query<(Entity, &Transform, &mut Sprite, &MechanicalBase), Without<Unit>>,
) {
    // Get the primary window and camera transform
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();
    
    // Process left mouse button click for selection
    if mouse_buttons.just_pressed(MouseButton::Left) {
        info!("Left mouse button clicked");
        
        if let Some(cursor_position) = window.cursor_position() {
            info!("Cursor position: {:?}", cursor_position);
            
            // First, clear any existing selections
            for (entity, _, _, _) in units.iter_mut() {
                commands.entity(entity).remove::<Selected>();
            }
            
            for (entity, _, _, _) in bases.iter_mut() {
                commands.entity(entity).remove::<Selected>();
            }
            
            // Convert screen position to world position
            if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                info!("World position: {:?}", world_position);
                
                // FIRST PRIORITY: Try to select mechanical bases
                for (entity, transform, _, _base) in bases.iter_mut() {
                    // Calculate distance to base center
                    let base_pos = transform.translation.truncate();
                    let distance = world_position.distance(base_pos);
                    
                    info!("Base at position: {:?}, distance from click: {}", base_pos, distance);
                    
                    // Use an extremely generous radius-based collision detection
                    // Making this very large to ensure bases are easy to select
                    let selection_radius = 128.0; // Super generous radius for much easier selection
                    
                    if distance < selection_radius {
                        // Select this base
                        commands.entity(entity).insert(Selected);
                        
                        // Early return - don't check anything else
                        return;
                    }
                }
                
                // SECOND PRIORITY: Start selection box for unit selection
                selection_box.start = Some(world_position);
                selection_box.end = Some(world_position);
                info!("Starting selection box at: {:?}", world_position);
            }
        }
    } else if mouse_buttons.pressed(MouseButton::Left) {
        // Update end position while dragging
        if let Some(cursor_position) = window.cursor_position() {
            if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                selection_box.end = Some(world_position);
            }
        }
    } else if mouse_buttons.just_released(MouseButton::Left) {
        // Finalize selection
        if let (Some(start), Some(end)) = (selection_box.start, selection_box.end) {
            let min_x = start.x.min(end.x);
            let max_x = start.x.max(end.x);
            let min_y = start.y.min(end.y);
            let max_y = start.y.max(end.y);
            
            // Select player units within the box
            for (entity, transform, _, unit) in units.iter_mut() {
                if unit.team == Team::Player {
                    let pos = transform.translation;
                    if pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y {
                        commands.entity(entity).insert(Selected);
                    }
                }
            }
            
            // Select player bases within the box
            for (entity, transform, _, base) in bases.iter_mut() {
                if base.team == Team::Player {
                    let pos = transform.translation;
                    if pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y {
                        commands.entity(entity).insert(Selected);
                    }
                }
            }
            
            // Reset selection box
            selection_box.start = None;
            selection_box.end = None;
        }
    }
    
    // Right-click to issue move orders (would be implemented here)
}

// System to update the visual representation of the selection box
fn update_selection_box_visual(
    selection_box: Res<SelectionBox>,
    mut commands: Commands,
    visual_query: Query<Entity, With<SelectionBoxVisual>>,
    _camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    // First, remove any existing selection box visual
    for entity in visual_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // If we're dragging (both start and end are Some), create a new selection box visual
    if let (Some(start), Some(end)) = (selection_box.start, selection_box.end) {
        // Calculate the box dimensions
        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);
        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);
        
        let width = max_x - min_x;
        let height = max_y - min_y;
        
        // Only show if it has some size (avoid flickering on small movements)
        if width > 5.0 || height > 5.0 {
            // Create a visual for the selection box
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.2, 0.6, 1.0, 0.3), // Semi-transparent blue
                        custom_size: Some(Vec2::new(width, height)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        min_x + width / 2.0,
                        min_y + height / 2.0,
                        100.0, // Above other entities
                    ),
                    ..default()
                },
                SelectionBoxVisual,
            ));
            
            // Also add a border around the selection box (more visible)
            // Top border
            spawn_selection_box_border(&mut commands, min_x, max_y, width, 2.0, true);
            // Bottom border
            spawn_selection_box_border(&mut commands, min_x, min_y, width, 2.0, true);
            // Left border
            spawn_selection_box_border(&mut commands, min_x, min_y, 2.0, height, false);
            // Right border
            spawn_selection_box_border(&mut commands, max_x, min_y, 2.0, height, false);
        }
    }
}

// Helper function to spawn a border segment for the selection box
fn spawn_selection_box_border(
    commands: &mut Commands, 
    x: f32, 
    y: f32, 
    width: f32, 
    height: f32, 
    horizontal: bool
) {
    let position_x = if horizontal { x + width / 2.0 } else { x };
    let position_y = if horizontal { y } else { y + height / 2.0 };
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.6, 1.0), // Solid blue
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(position_x, position_y, 101.0), // Above the selection box
            ..default()
        },
        SelectionBoxVisual,
    ));
}

// System to update the visual appearance of selected entities
fn update_selection_visuals(
    selected_bases: Query<Entity, (With<Selected>, With<MechanicalBase>)>,
    selected_units: Query<Entity, (With<Selected>, With<Unit>)>,
    mut sprites: Query<(&mut Sprite, Option<&MechanicalBase>, Option<&Unit>)>,
) {
    // Update appearances of all sprites
    for (mut sprite, base, unit) in sprites.iter_mut() {
        let entity_selected = match (base, unit) {
            (Some(_), _) => selected_bases.iter().count() > 0,
            (_, Some(_)) => selected_units.iter().count() > 0,
            _ => false,
        };
        
        if entity_selected {
            // If this entity is selected, make it bright blue
            sprite.color = Color::srgb(0.4, 0.8, 1.0);
        } else {
            // Otherwise, use default color based on team
            if let Some(base) = base {
                match base.team {
                    Team::Player => sprite.color = Color::srgb(0.2, 0.6, 0.8), // Blue for player
                    Team::Enemy => sprite.color = Color::srgb(0.8, 0.2, 0.2),  // Red for enemy
                    _ => {}
                }
            } else if let Some(unit) = unit {
                match unit.team {
                    Team::Player => sprite.color = Color::srgb(0.2, 0.2, 0.8), // Darker blue for player units
                    Team::Enemy => sprite.color = Color::srgb(0.8, 0.2, 0.2),  // Red for enemy units
                    _ => {}
                }
            }
        }
    }
}

fn handle_game_over_condition(
    units: Query<&Unit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Count player and enemy units
    let mut player_count = 0;
    let mut enemy_count = 0;

    for unit in units.iter() {
        match unit.team {
            Team::Player => player_count += 1,
            Team::Enemy => enemy_count += 1,
            _ => {}
        }
    }

    // Check win/loss conditions
    if player_count == 0 {
        // Player lost all units
        next_state.set(GameState::GameOver);
    } else if enemy_count == 0 {
        // Player defeated all enemies
        next_state.set(GameState::GameOver);
    }
}

fn cleanup_gameplay(
    mut commands: Commands,
    ui_query: Query<Entity, With<GameplayUI>>,
    selection_box_query: Query<Entity, With<SelectionBoxVisual>>,
    unit_query: Query<Entity, With<Unit>>,
) {
    // Remove UI elements
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Remove any selection box visuals
    for entity in selection_box_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Remove all units
    for entity in unit_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Remove resources
    commands.remove_resource::<GameResources>();
    commands.remove_resource::<SelectionBox>();
}
