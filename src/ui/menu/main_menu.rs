use bevy::prelude::*;
use crate::states::game_state::GameState;
use super::components::{MenuUI, create_button, create_title};

/// Plugin for the main menu
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
           .add_systems(Update, handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)))
           .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}

/// Marker component for main menu UI elements
#[derive(Component)]
struct MainMenuUI;

/// Main menu button types
#[derive(Component)]
enum MainMenuButton {
    Campaign,
    Skirmish,
    Multiplayer,
    FactionHQ,
    Settings,
    Profile,
    Extras,
    Quit,
}

/// Set up the main menu UI
fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Camera is now managed by the CameraManagerPlugin
    
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
            MenuUI,
        ))
        .with_children(|parent| {
            // Title
            create_title(parent, "STRATEGY FORGE", &asset_server, 80.0);
            
            // Version info
            parent.spawn(TextBundle::from_section(
                "Version 0.1.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::srgba(0.7, 0.7, 0.7, 1.0),
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
                    create_button(parent, "Campaign", MainMenuButton::Campaign, &asset_server, 250.0, 65.0);
                    
                    // Skirmish button
                    create_button(parent, "Skirmish", MainMenuButton::Skirmish, &asset_server, 250.0, 65.0);
                    
                    // Multiplayer button
                    create_button(parent, "Multiplayer", MainMenuButton::Multiplayer, &asset_server, 250.0, 65.0);
                    
                    // Faction HQ button
                    create_button(parent, "Faction Headquarters", MainMenuButton::FactionHQ, &asset_server, 250.0, 65.0);
                    
                    // Settings button
                    create_button(parent, "Settings", MainMenuButton::Settings, &asset_server, 250.0, 65.0);
                    
                    // Profile button
                    create_button(parent, "Profile", MainMenuButton::Profile, &asset_server, 250.0, 65.0);
                    
                    // Extras button
                    create_button(parent, "Extras", MainMenuButton::Extras, &asset_server, 250.0, 65.0);
                    
                    // Quit button
                    create_button(parent, "Quit", MainMenuButton::Quit, &asset_server, 250.0, 65.0);
                });
            
            // Footer with copyright info
            parent.spawn(TextBundle::from_section(
                "© 2025 StrategyForge Team",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::srgba(0.6, 0.6, 0.6, 1.0),
                },
            ));
        });
}

/// Handle button interactions in the main menu
fn handle_main_menu_buttons(
    mut button_query: Query<
        (&Interaction, &MainMenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut campaign_events: EventWriter<super::campaign_menu::OpenCampaignMenuEvent>,
    mut skirmish_events: EventWriter<super::skirmish_menu::OpenSkirmishMenuEvent>,
    mut faction_events: EventWriter<super::faction_menu::OpenFactionMenuEvent>,
    mut settings_events: EventWriter<super::settings_menu::OpenSettingsEvent>,
    mut profile_events: EventWriter<super::profile_menu::OpenProfileMenuEvent>,
    mut extras_events: EventWriter<super::extras_menu::OpenExtrasMenuEvent>,
) {
    for (interaction, button_type, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    MainMenuButton::Campaign => {
                        println!("Campaign button pressed!");
                        campaign_events.send(super::campaign_menu::OpenCampaignMenuEvent);
                    }
                    MainMenuButton::Skirmish => {
                        println!("Skirmish button pressed!");
                        skirmish_events.send(super::skirmish_menu::OpenSkirmishMenuEvent);
                    }
                    MainMenuButton::Multiplayer => {
                        println!("Multiplayer button pressed!");
                        // For now, just go to gameplay as a placeholder
                        next_state.set(GameState::Gameplay);
                    }
                    MainMenuButton::FactionHQ => {
                        println!("Faction HQ button pressed!");
                        faction_events.send(super::faction_menu::OpenFactionMenuEvent);
                    }
                    MainMenuButton::Settings => {
                        println!("Settings button pressed!");
                        settings_events.send(super::settings_menu::OpenSettingsEvent);
                    }
                    MainMenuButton::Profile => {
                        println!("Profile button pressed!");
                        profile_events.send(super::profile_menu::OpenProfileMenuEvent);
                    }
                    MainMenuButton::Extras => {
                        println!("Extras button pressed!");
                        extras_events.send(super::extras_menu::OpenExtrasMenuEvent);
                    }
                    MainMenuButton::Quit => {
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

/// Clean up the main menu when exiting
fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    // Remove main menu UI
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Camera cleanup is now handled by the CameraManagerPlugin
}
