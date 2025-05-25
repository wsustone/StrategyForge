use bevy::prelude::*;
use bevy::app::AppExit;
use crate::states::game_state::GameState;
use crate::ui::menu::{OpenSettingsEvent, OpenFactionMenuEvent};
use crate::utils::font_loader::get_font_handle;
use crate::systems::camera_manager::spawn_camera_for_state;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
           .add_systems(Update, handle_menu_buttons.run_if(in_state(GameState::MainMenu)))
           .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}

// UI Component markers
#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
enum MenuButton {
    Campaign,
    Skirmish,
    Multiplayer,
    FactionHQ,
    Settings,
    Profile,
    Extras,
    Quit,
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Set up camera with state management
    spawn_camera_for_state(&mut commands, GameState::MainMenu);
    
    // Root node
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
                background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.3)),
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "STRATEGY FORGE",
                TextStyle {
                    font: get_font_handle(&asset_server),
                    font_size: 80.0,
                    color: Color::srgba(0.9, 0.9, 0.9, 1.0),
                },
            ));
            
            // Buttons container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(50.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Campaign button
                    spawn_button(parent, "Campaign", MenuButton::Campaign, &asset_server);
                    
                    // Skirmish button
                    spawn_button(parent, "Skirmish", MenuButton::Skirmish, &asset_server);
                    
                    // Multiplayer button
                    spawn_button(parent, "Multiplayer", MenuButton::Multiplayer, &asset_server);
                    
                    // Faction HQ button
                    spawn_button(parent, "Faction HQ", MenuButton::FactionHQ, &asset_server);
                    
                    // Settings button
                    spawn_button(parent, "Settings", MenuButton::Settings, &asset_server);
                    
                    // Profile button
                    spawn_button(parent, "Profile", MenuButton::Profile, &asset_server);
                    
                    // Extras button
                    spawn_button(parent, "Extras", MenuButton::Extras, &asset_server);
                    
                    // Quit button
                    spawn_button(parent, "Quit", MenuButton::Quit, &asset_server);
                });
        });
}

fn spawn_button(
    parent: &mut ChildBuilder,
    text: &str,
    button_type: MenuButton,
    asset_server: &Res<AssetServer>,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.35)),
                ..default()
            },
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: get_font_handle(asset_server),
                    font_size: 30.0,
                    color: Color::srgba(0.9, 0.9, 0.9, 1.0),
                },
            ));
        });
}

fn handle_menu_buttons(
    mut button_query: Query<
        (&Interaction, &MenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut settings_events: EventWriter<OpenSettingsEvent>,
    mut faction_events: EventWriter<OpenFactionMenuEvent>,
) {
    for (interaction, button_type, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    MenuButton::Campaign => {
                        println!("Campaign button pressed!");
                        // For now, just go to gameplay
                        next_state.set(GameState::Gameplay);
                    }
                    MenuButton::Skirmish => {
                        println!("Skirmish button pressed!");
                        // For now, just go to gameplay
                        next_state.set(GameState::Gameplay);
                    }
                    MenuButton::Multiplayer => {
                        println!("Multiplayer button pressed!");
                        // For now, just go to gameplay as a placeholder
                        next_state.set(GameState::Gameplay);
                    }
                    MenuButton::FactionHQ => {
                        println!("Faction HQ button pressed!");
                        // Send event to open the faction menu
                        faction_events.send(OpenFactionMenuEvent);
                    }
                    MenuButton::Settings => {
                        println!("Settings button pressed!");
                        // Send event to open the settings menu
                        settings_events.send(OpenSettingsEvent);
                    }
                    MenuButton::Profile => {
                        println!("Profile button pressed!");
                        // For now, just print to console
                    }
                    MenuButton::Extras => {
                        println!("Extras button pressed!");
                        // For now, just print to console
                    }
                    MenuButton::Quit => {
                        println!("Quit button pressed!");
                        app_exit_events.send(AppExit::default());
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

fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    // Remove main menu UI
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Remove camera
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
