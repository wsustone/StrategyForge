use bevy::prelude::*;
use crate::states::game_state::GameState;
use crate::systems::camera_manager::spawn_camera_for_state;
use crate::utils::font_loader::get_font_handle;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_game_over)
           .add_systems(Update, handle_game_over_input.run_if(in_state(GameState::GameOver)))
           .add_systems(OnExit(GameState::GameOver), cleanup_game_over);
    }
}

// UI Component markers
#[derive(Component)]
struct GameOverUI;

fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Set up camera with state management
    spawn_camera_for_state(&mut commands, GameState::GameOver);
    
    // Game over screen
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.1, 0.0, 0.0)),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            // Game Over text
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font: get_font_handle(&asset_server),
                    font_size: 100.0,
                    color: Color::srgba(0.9, 0.1, 0.1, 1.0),
                },
            ));
            
            // Message
            parent.spawn(TextBundle::from_section(
                "Your base has been destroyed!",
                TextStyle {
                    font: get_font_handle(&asset_server),
                    font_size: 36.0,
                    color: Color::srgba(0.9, 0.6, 0.6, 1.0),
                },
            ));
            
            // Return to main menu button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(30.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.3, 0.1, 0.1)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Return to Main Menu",
                        TextStyle {
                            font: get_font_handle(&asset_server),
                            font_size: 24.0,
                            color: Color::srgba(0.9, 0.9, 0.9, 1.0),
                        },
                    ));
                });
        });
}

fn handle_game_over_input(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Return to main menu on button click
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::MainMenu);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgba(0.4, 0.2, 0.2, 1.0));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgba(0.3, 0.1, 0.1, 1.0));
            }
        }
    }
    
    // Also return to main menu on Escape key
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

fn cleanup_game_over(
    mut commands: Commands,
    ui_query: Query<Entity, With<GameOverUI>>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    // Remove the game over UI
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Remove camera
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
