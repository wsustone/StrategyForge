use bevy::prelude::*;
use crate::GameState;
use crate::utils::font_loader::get_font_handle;
use crate::components::unit::{Unit, Team};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), setup_gameplay)
           .add_systems(
                Update,
                (
                    handle_input,
                    unit_selection,
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

fn spawn_example_units(commands: &mut Commands) {
    // Player units (blue)
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.2, 0.2, 0.8),
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-100.0 + i as f32 * 30.0, -100.0, 0.0),
                ..default()
            },
            Unit {
                health: 100.0,
                max_health: 100.0,
                attack_power: 15.0,
                attack_range: 50.0,
                movement_speed: 100.0,
                team: Team::Player,
            },
        ));
    }
    
    // Enemy units (red)
    for i in 0..3 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.8, 0.2, 0.2),
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-50.0 + i as f32 * 40.0, 100.0, 0.0),
                ..default()
            },
            Unit {
                health: 80.0,
                max_health: 80.0,
                attack_power: 10.0,
                attack_range: 40.0,
                movement_speed: 90.0,
                team: Team::Enemy,
            },
        ));
    }
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

fn unit_selection(
    mut commands: Commands,
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut selection_box: ResMut<SelectionBox>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut units: Query<(Entity, &Transform, &mut Sprite, &Unit)>,
) {
    // Get the primary window
    let window = windows.single();
    
    // Get the camera transform
    let (camera, camera_transform) = camera_q.single();
    
    if mouse_buttons.just_pressed(MouseButton::Left) {
        // Start selection
        if let Some(cursor_position) = window.cursor_position() {
            // First, clear any existing selections
            for (entity, _, mut sprite, _) in units.iter_mut() {
                commands.entity(entity).remove::<Selected>();
                sprite.color = sprite.color.with_alpha(1.0); // Reset alpha
            }
            
            // Convert screen position to world position
            if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                selection_box.start = Some(world_position);
                selection_box.end = Some(world_position);
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
            for (entity, transform, mut sprite, unit) in units.iter_mut() {
                if unit.team == Team::Player {
                    let pos = transform.translation;
                    if pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y {
                        commands.entity(entity).insert(Selected);
                        sprite.color = sprite.color.with_alpha(1.5); // Highlight selected units
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

fn handle_game_over_condition(
    units: Query<&Unit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Count units by team
    let mut player_units = 0;
    let mut enemy_units = 0;
    
    for unit in units.iter() {
        match unit.team {
            Team::Player => player_units += 1,
            Team::Enemy => enemy_units += 1,
            Team::Neutral => {}, // Neutral units don't affect victory conditions
        }
    }
    
    // If player has no units left, game over
    if player_units == 0 {
        next_state.set(GameState::GameOver);
    }
    
    // If enemy has no units left, player wins (would transition to a victory state)
    if enemy_units == 0 {
        // For now, just return to main menu
        next_state.set(GameState::MainMenu);
    }
}

fn cleanup_gameplay(
    mut commands: Commands,
    ui_query: Query<Entity, With<GameplayUI>>,
    unit_query: Query<Entity, With<Unit>>,
) {
    // Remove UI
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Remove all units
    for entity in unit_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Remove resources
    commands.remove_resource::<GameResources>();
    commands.remove_resource::<SelectionBox>();
}
