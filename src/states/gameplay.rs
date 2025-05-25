use bevy::prelude::*;
use crate::states::game_state::GameState;
use crate::utils::font_loader::get_font_handle;
use crate::components::unit::{Unit, Team};
use crate::components::unit_types::UnitType;
use crate::components::base_modules::ResourceType;
use crate::components::player::{MechanicalBase, PlayerResources};
use crate::sprites::GameSprites;
use crate::systems::camera_manager::spawn_camera_for_state;

// TODO: Move UnitType to components/unit.rs or create a proper unit_types module

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
    _gold: i32,
    _wood: i32,
    _stone: i32,
}

#[derive(Resource)]
struct SelectionBox {
    start: Option<Vec2>,
    end: Option<Vec2>,
}

fn setup_gameplay(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    game_sprites: Res<GameSprites>,
) {
    // Set up camera with state management
    spawn_camera_for_state(&mut commands, GameState::Gameplay);
    
    // Initialize game resources
    commands.insert_resource(GameResources {
        _gold: 500,
        _wood: 300,
        _stone: 200,
    });
    
    // Initialize player resources for production and economy
    commands.insert_resource(PlayerResources::default());
    
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
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ..default()
            })
            .with_children(|parent| {
                // Gold
                parent.spawn(TextBundle::from_section(
                    "Gold: 500",
                    TextStyle {
                        font: get_font_handle(&asset_server),
                        font_size: 20.0,
                        color: Color::srgba(1.0, 0.9, 0.0, 1.0),
                    },
                ));
                
                // Wood
                parent.spawn(TextBundle::from_section(
                    "Wood: 300",
                    TextStyle {
                        font: get_font_handle(&asset_server),
                        font_size: 20.0,
                        color: Color::srgba(0.6, 0.4, 0.2, 1.0),
                    },
                ));
                
                // Stone
                parent.spawn(TextBundle::from_section(
                    "Stone: 200",
                    TextStyle {
                        font: get_font_handle(&asset_server),
                        font_size: 20.0,
                        color: Color::srgba(0.7, 0.7, 0.7, 1.0),
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
                background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                ..default()
            });
        });
    
    // Spawn example units with proper sprites
    spawn_example_units(&mut commands, &game_sprites);
    
    // Spawn mechanical bases with proper sprites
    spawn_mechanical_bases(&mut commands, &game_sprites);
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

fn spawn_example_units(commands: &mut Commands, game_sprites: &Res<GameSprites>) {
    // Spawn specific unit types that will use our sprites
    // 2 tanks and 2 artillery units for each team
    
    // Spawn player's units
    // Player tanks
    for i in 0..2 {
        let unit_type = UnitType::LandToLandTank;
        let position = Vec2::new(-200.0 + i as f32 * 50.0, -150.0);
        
        // Get the tank sprite from the GameSprites resource
        let sprite_handle = if game_sprites.is_loaded {
            game_sprites.get_unit_sprite("tank_player", 0).cloned()
        } else {
            None
        };
        
        // Spawn the tank
        let tank_entity = unit_type.spawn_unit(commands, position, Team::Player);
        
        // If we have a sprite, attach it to the entity
        if let Some(texture) = sprite_handle {
            commands.entity(tank_entity).insert(texture);
        }
        
        commands.entity(tank_entity).insert(Name::new(format!("Player Tank {}", i)));
    }
    
    // Player artillery
    for i in 0..2 {
        let unit_type = UnitType::Artillery;
        let position = Vec2::new(-200.0 + i as f32 * 50.0, -200.0);
        
        // Get the artillery sprite from the GameSprites resource
        let sprite_handle = if game_sprites.is_loaded {
            game_sprites.get_unit_sprite("artillery_player", 0).cloned()
        } else {
            None
        };
        
        // Spawn the artillery
        let artillery_entity = unit_type.spawn_unit(commands, position, Team::Player);
        
        // If we have a sprite, attach it to the entity
        if let Some(texture) = sprite_handle {
            commands.entity(artillery_entity).insert(texture);
        }
        
        commands.entity(artillery_entity).insert(Name::new(format!("Player Artillery {}", i)));
    }
    
    // Spawn enemy units
    // Enemy tanks
    for i in 0..2 {
        let unit_type = UnitType::LandToLandTank;
        let position = Vec2::new(200.0 - i as f32 * 50.0, 150.0);
        
        // Get the tank sprite from the GameSprites resource - use direction 4 (facing left)
        let sprite_handle = if game_sprites.is_loaded {
            game_sprites.get_unit_sprite("tank_enemy", 4).cloned()
        } else {
            None
        };
        
        // Spawn the tank
        let tank_entity = unit_type.spawn_unit(commands, position, Team::Enemy);
        
        // If we have a sprite, attach it to the entity
        if let Some(texture) = sprite_handle {
            commands.entity(tank_entity).insert(texture);
        }
        
        commands.entity(tank_entity).insert(Name::new(format!("Enemy Tank {}", i)));
    }
    
    // Enemy artillery
    for i in 0..2 {
        let unit_type = UnitType::Artillery;
        let position = Vec2::new(200.0 - i as f32 * 50.0, 200.0);
        
        // Get the artillery sprite from the GameSprites resource - use direction 4 (facing left)
        let sprite_handle = if game_sprites.is_loaded {
            game_sprites.get_unit_sprite("artillery_enemy", 4).cloned()
        } else {
            None
        };
        
        // Spawn the artillery
        let artillery_entity = unit_type.spawn_unit(commands, position, Team::Enemy);
        
        // If we have a sprite, attach it to the entity
        if let Some(texture) = sprite_handle {
            commands.entity(artillery_entity).insert(texture);
        }
        
        commands.entity(artillery_entity).insert(Name::new(format!("Enemy Artillery {}", i)));
    }
    
    info!("Spawned tanks and artillery units");
}

// Function to spawn mechanical bases
fn spawn_mechanical_bases(commands: &mut Commands, game_sprites: &Res<GameSprites>) {
    // Spawn player's mechanical base with sprite
    let player_base_sprite = if game_sprites.is_loaded {
        // Use direction 0 (facing right)
        game_sprites.get_base_sprite("base_player", 0).cloned()
    } else {
        None
    };
    
    commands.spawn((
        if let Some(texture) = player_base_sprite {
            // Use the base sprite
            SpriteBundle {
                texture,
                sprite: Sprite {
                    // Still apply a slight blue tint to indicate player team
                    color: Color::srgba(0.9, 0.9, 1.0, 1.0),
                    // Size will be determined by the sprite
                    ..default()
                },
                transform: Transform::from_xyz(-250.0, -250.0, 1.0) // Low z value to ensure visibility
                    .with_scale(Vec3::new(0.25, 0.25, 1.0)), // Scale down to 25% size
                ..default()
            }
        } else {
            // Fallback to colored square if sprite not loaded
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.2, 0.6, 0.8, 1.0), // Blue for player
                    custom_size: Some(Vec2::new(40.0, 40.0)), // Larger size for the base
                    ..default()
                },
                transform: Transform::from_xyz(-250.0, -250.0, 1.0),
                ..default()
            }
        },
        MechanicalBase {
            health: 1000.0,
            max_health: 1000.0,
            base_movement_speed: 50.0,
            effective_movement_speed: 50.0,
            team: Team::Player,
            resources: vec![
                (ResourceType::Wood, 100),
                (ResourceType::Stone, 50),
                (ResourceType::Iron, 25),
            ],
            power_output: 100.0,
            power_consumed: 0.0,
            max_power: 150.0,
            attachment_points: Vec::new(),
            modules: Vec::new(),
        },
        Name::new("Player Base"),
    ));
    
    // Spawn enemy's mechanical base with sprite
    let enemy_base_sprite = if game_sprites.is_loaded {
        // Use direction 4 (facing left) for enemy base
        game_sprites.get_base_sprite("base_enemy", 4).cloned()
    } else {
        None
    };
    
    commands.spawn((
        if let Some(texture) = enemy_base_sprite {
            // Use the base sprite
            SpriteBundle {
                texture,
                sprite: Sprite {
                    // Apply a red tint to indicate enemy team
                    color: Color::srgba(1.0, 0.8, 0.8, 1.0),
                    // Size will be determined by the sprite
                    ..default()
                },
                transform: Transform::from_xyz(250.0, 250.0, 1.0) // Low z value to ensure visibility
                    .with_scale(Vec3::new(0.25, 0.25, 1.0)), // Scale down to 25% size
                ..default()
            }
        } else {
            // Fallback to colored square if sprite not loaded
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.8, 0.2, 0.2, 1.0), // Red for enemy
                    custom_size: Some(Vec2::new(40.0, 40.0)), // Larger size for the base
                    ..default()
                },
                transform: Transform::from_xyz(250.0, 250.0, 1.0),
                ..default()
            }
        },
        MechanicalBase {
            health: 1000.0,
            max_health: 1000.0,
            base_movement_speed: 50.0,
            effective_movement_speed: 50.0,
            team: Team::Enemy,
            resources: vec![
                (ResourceType::Wood, 100),
                (ResourceType::Stone, 50),
                (ResourceType::Iron, 25),
            ],
            power_output: 100.0,
            power_consumed: 0.0,
            max_power: 150.0,
            attachment_points: Vec::new(),
            modules: Vec::new(),
        },
        Name::new("Enemy Base"),
    ));
    
    info!("Mechanical bases spawned");
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
                    
                    // Use a reasonable radius-based collision detection
                    // Base size is 40x40, so we'll use half that plus a bit extra
                    let selection_radius = 25.0; // More reasonable radius for base selection
                    
                    if distance < selection_radius {
                        // Select this base
                        commands.entity(entity).insert(Selected);
                        info!("Selected base at {:?}", base_pos);
                        
                        // Early return - don't try to select other bases
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
            
            // Check if the selection box is large enough to be a deliberate selection
            // If it's too small, it might be just a click with minimal movement
            let box_width = max_x - min_x;
            let box_height = max_y - min_y;
            let min_selection_size = 5.0; // Minimum size for box selection to activate
            
            if box_width > min_selection_size && box_height > min_selection_size {
                info!("Selection box: ({}, {}) to ({}, {})", min_x, min_y, max_x, max_y);
                
                // Select player units within the box
                for (entity, transform, _, unit) in units.iter_mut() {
                    if unit.team == Team::Player {
                        let pos = transform.translation;
                        if pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y {
                            commands.entity(entity).insert(Selected);
                            info!("Selected unit at ({}, {})", pos.x, pos.y);
                        }
                    }
                }
            } else {
                // It was just a click, try to select a single unit or base
                let click_pos = Vec2::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
                info!("Treating as single click at ({}, {})", click_pos.x, click_pos.y);
                
                // Try to select a unit first
                let mut closest_unit = None;
                let mut closest_distance = f32::MAX;
                let selection_radius = 15.0; // Radius for single unit selection
                
                // First pass: check for units
                for (entity, transform, _, unit) in units.iter_mut() {
                    if unit.team == Team::Player {
                        let pos = transform.translation.truncate();
                        let distance = click_pos.distance(pos);
                        
                        if distance < selection_radius && distance < closest_distance {
                            closest_unit = Some(entity);
                            closest_distance = distance;
                        }
                    }
                }
                
                // If we found a unit to select, select it
                if let Some(entity) = closest_unit {
                    commands.entity(entity).insert(Selected);
                    info!("Selected closest unit at distance {}", closest_distance);
                } else {
                    // If no unit was found, try to select a base
                    let mut closest_base = None;
                    closest_distance = f32::MAX; // Reset for base selection
                    let base_selection_radius = 25.0;
                    
                    for (entity, transform, _, base) in bases.iter_mut() {
                        if base.team == Team::Player {
                            let pos = transform.translation.truncate();
                            let distance = click_pos.distance(pos);
                            
                            if distance < base_selection_radius && distance < closest_distance {
                                closest_base = Some(entity);
                                closest_distance = distance;
                            }
                        }
                    }
                    
                    // Select only the closest base
                    if let Some(entity) = closest_base {
                        commands.entity(entity).insert(Selected);
                        info!("Selected closest base at distance {}", closest_distance);
                    }
                }
            }
            
            // If we're doing box selection, also check for bases
            if box_width > min_selection_size && box_height > min_selection_size {
                // Select player bases within the box
                for (entity, transform, _, base) in bases.iter_mut() {
                    if base.team == Team::Player {
                        let pos = transform.translation;
                        if pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y {
                            commands.entity(entity).insert(Selected);
                            info!("Selected base at ({}, {})", pos.x, pos.y);
                        }
                    }
                }
            }
            // Note: The single-click base selection is now handled in the unit selection 'else' block above
            
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
                        color: Color::srgba(0.2, 0.6, 1.0, 0.3),
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
                color: Color::srgba(0.2, 0.6, 1.0, 1.0),
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
            sprite.color = Color::srgba(0.4, 0.8, 1.0, 1.0);
        } else {
            // Otherwise, use default color based on team
            if let Some(base) = base {
                match base.team {
                    Team::Player => sprite.color = Color::srgba(0.2, 0.6, 0.8, 1.0), // Blue for player
                    Team::Enemy => sprite.color = Color::srgba(0.8, 0.2, 0.2, 1.0),  // Red for enemy
                    _ => {}
                }
            } else if let Some(unit) = unit {
                match unit.team {
                    Team::Player => sprite.color = Color::srgba(0.2, 0.2, 0.8, 1.0), // Darker blue for player units
                    Team::Enemy => sprite.color = Color::srgba(0.8, 0.2, 0.2, 1.0),  // Red for enemy units
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

