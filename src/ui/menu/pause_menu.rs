use bevy::prelude::*;
use crate::states::game_state::GameState;
use super::components::{MenuUI, create_button, create_title};

/// Placeholder for tech tree event until the tech module is fully integrated
#[derive(Event)]
pub struct OpenTechTreeEvent;

/// Plugin for the pause menu
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), setup_pause_menu)
           .add_systems(Update, handle_pause_menu_buttons.run_if(in_state(GameState::Paused)))
           .add_systems(OnExit(GameState::Paused), cleanup_pause_menu)
           .add_systems(Update, toggle_pause.run_if(in_state(GameState::Gameplay)));
    }
}

/// Marker component for pause menu UI elements
#[derive(Component)]
struct PauseMenuUI;

/// Pause menu button types
#[derive(Component)]
enum PauseMenuButton {
    Resume,
    SaveGame,
    LoadGame,
    Settings,
    Objectives,
    TechTree,
    Help,
    RestartMission,
    ExitToMainMenu,
}

/// System to toggle pause state with the Escape key
fn toggle_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

/// Set up the pause menu UI
fn setup_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Root node (semi-transparent overlay)
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                z_index: ZIndex::Global(10), // Make sure it appears on top
                ..default()
            },
            PauseMenuUI,
            MenuUI,
        ))
        .with_children(|parent| {
            // Pause menu panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(15.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.35)),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    create_title(parent, "GAME PAUSED", &asset_server, 40.0);
                    
                    // Resume button
                    create_button(parent, "Resume Game", PauseMenuButton::Resume, &asset_server, 300.0, 50.0);
                    
                    // Save Game button
                    create_button(parent, "Save Game", PauseMenuButton::SaveGame, &asset_server, 300.0, 50.0);
                    
                    // Load Game button
                    create_button(parent, "Load Game", PauseMenuButton::LoadGame, &asset_server, 300.0, 50.0);
                    
                    // Settings button
                    create_button(parent, "Settings", PauseMenuButton::Settings, &asset_server, 300.0, 50.0);
                    
                    // Mission Objectives button
                    create_button(parent, "Mission Objectives", PauseMenuButton::Objectives, &asset_server, 300.0, 50.0);
                    
                    // Tech Tree button
                    create_button(parent, "Technology Tree", PauseMenuButton::TechTree, &asset_server, 300.0, 50.0);
                    
                    // Help button
                    create_button(parent, "Help", PauseMenuButton::Help, &asset_server, 300.0, 50.0);
                    
                    // Restart Mission button
                    create_button(parent, "Restart Mission", PauseMenuButton::RestartMission, &asset_server, 300.0, 50.0);
                    
                    // Exit to Main Menu button
                    create_button(parent, "Exit to Main Menu", PauseMenuButton::ExitToMainMenu, &asset_server, 300.0, 50.0);
                });
        });
}

/// Handle button interactions in the pause menu
fn handle_pause_menu_buttons(
    mut button_query: Query<
        (&Interaction, &PauseMenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings_events: EventWriter<super::settings_menu::OpenSettingsEvent>,
    mut tech_events: EventWriter<OpenTechTreeEvent>,
) {
    for (interaction, button_type, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    PauseMenuButton::Resume => {
                        next_state.set(GameState::Gameplay);
                    }
                    PauseMenuButton::SaveGame => {
                        println!("Save Game button pressed!");
                        // Save game functionality would go here
                        // For now, just return to gameplay
                        next_state.set(GameState::Gameplay);
                    }
                    PauseMenuButton::LoadGame => {
                        println!("Load Game button pressed!");
                        // Load game functionality would go here
                        // For now, just return to gameplay
                        next_state.set(GameState::Gameplay);
                    }
                    PauseMenuButton::Settings => {
                        println!("Settings button pressed!");
                        // First return to gameplay, then open settings overlay
                        next_state.set(GameState::Gameplay);
                        settings_events.send(super::settings_menu::OpenSettingsEvent);
                    }
                    PauseMenuButton::Objectives => {
                        println!("Objectives button pressed!");
                        // Mission objectives functionality would go here
                        // For now, just return to gameplay
                        next_state.set(GameState::Gameplay);
                    }
                    PauseMenuButton::TechTree => {
                        println!("Tech Tree button pressed!");
                        // Open the tech tree UI
                        next_state.set(GameState::Gameplay);
                        tech_events.send(OpenTechTreeEvent);
                    }
                    PauseMenuButton::Help => {
                        println!("Help button pressed!");
                        // Help functionality would go here
                        // For now, just return to gameplay
                        next_state.set(GameState::Gameplay);
                    }
                    PauseMenuButton::RestartMission => {
                        println!("Restart Mission button pressed!");
                        // Restart mission functionality would go here
                        // For now, just return to gameplay
                        next_state.set(GameState::Gameplay);
                    }
                    PauseMenuButton::ExitToMainMenu => {
                        println!("Exit to Main Menu button pressed!");
                        next_state.set(GameState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::srgb(0.25, 0.25, 0.45));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.35));
            }
        }
    }
}

/// Clean up the pause menu when exiting
fn cleanup_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenuUI>>,
) {
    // Remove pause menu UI
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
