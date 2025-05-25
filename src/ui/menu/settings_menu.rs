use bevy::prelude::*;
use super::components::{MenuUI, create_button, create_title};

/// Plugin for the settings menu
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenSettingsEvent>()
           .add_event::<CloseSettingsEvent>()
           .add_systems(Update, handle_open_settings_event)
           .add_systems(Update, handle_settings_buttons.run_if(resource_exists::<SettingsMenuState>))
           .add_systems(Update, handle_close_settings_event);
    }
}

/// Event to open the settings menu
#[derive(Event)]
pub struct OpenSettingsEvent;

/// Event to close the settings menu
#[derive(Event)]
pub struct CloseSettingsEvent;

/// State of the settings menu
#[derive(Resource)]
struct SettingsMenuState {
    active_tab: SettingsTab,
}

/// Tabs in the settings menu
#[derive(PartialEq, Eq, Clone, Copy)]
enum SettingsTab {
    Video,
    Audio,
    Controls,
    Gameplay,
    Interface,
}

/// Marker component for settings menu UI elements
#[derive(Component)]
struct SettingsMenuUI;

/// Settings menu button types
#[derive(Component)]
enum SettingsButton {
    VideoTab,
    AudioTab,
    ControlsTab,
    GameplayTab,
    InterfaceTab,
    Apply,
    Reset,
    Back,
}

/// Slider component for settings
#[derive(Component)]
struct SettingsSlider {
    setting_type: SliderType,
    min: f32,
    max: f32,
    current: f32,
}

/// Types of sliders in the settings menu
enum SliderType {
    MasterVolume,
    MusicVolume,
    SfxVolume,
    VoiceVolume,
    AmbientVolume,
    MouseSensitivity,
    GameSpeed,
    UiScale,
}

/// Handle the open settings event
fn handle_open_settings_event(
    mut commands: Commands,
    mut ev_open_settings: EventReader<OpenSettingsEvent>,
    asset_server: Res<AssetServer>,
    settings_state: Option<Res<SettingsMenuState>>,
) {
    for _ in ev_open_settings.read() {
        // Only open settings if it's not already open
        if settings_state.is_none() {
            commands.insert_resource(SettingsMenuState {
                active_tab: SettingsTab::Video,
            });
            
            spawn_settings_menu(&mut commands, &asset_server);
        }
    }
}

/// Handle the close settings event
fn handle_close_settings_event(
    mut commands: Commands,
    mut ev_close_settings: EventReader<CloseSettingsEvent>,
    settings_ui_query: Query<Entity, With<SettingsMenuUI>>,
    settings_state: Option<Res<SettingsMenuState>>,
) {
    for _ in ev_close_settings.read() {
        if settings_state.is_some() {
            // Remove settings UI
            for entity in settings_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            
            // Remove settings state
            commands.remove_resource::<SettingsMenuState>();
        }
    }
}

/// Spawn the settings menu UI
fn spawn_settings_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    // Root node
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
            SettingsMenuUI,
            MenuUI,
        ))
        .with_children(|parent| {
            // Settings panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(800.0),
                        height: Val::Px(600.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.35)),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    create_title(parent, "SETTINGS", asset_server, 40.0);
                    
                    // Tabs
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(20.0), Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Video tab
                            create_button(parent, "Video", SettingsButton::VideoTab, asset_server, 150.0, 40.0);
                            
                            // Audio tab
                            create_button(parent, "Audio", SettingsButton::AudioTab, asset_server, 150.0, 40.0);
                            
                            // Controls tab
                            create_button(parent, "Controls", SettingsButton::ControlsTab, asset_server, 150.0, 40.0);
                            
                            // Gameplay tab
                            create_button(parent, "Gameplay", SettingsButton::GameplayTab, asset_server, 150.0, 40.0);
                            
                            // Interface tab
                            create_button(parent, "Interface", SettingsButton::InterfaceTab, asset_server, 150.0, 40.0);
                        });
                    
                    // Content area
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(70.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Default to video settings
                            spawn_video_settings(parent, asset_server);
                        });
                    
                    // Bottom buttons
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceEvenly,
                                margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(20.0), Val::Px(0.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Apply button
                            create_button(parent, "Apply", SettingsButton::Apply, asset_server, 150.0, 40.0);
                            
                            // Reset button
                            create_button(parent, "Reset", SettingsButton::Reset, asset_server, 150.0, 40.0);
                            
                            // Back button
                            create_button(parent, "Back", SettingsButton::Back, asset_server, 150.0, 40.0);
                        });
                });
        });
}

/// Spawn video settings content
fn spawn_video_settings(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Resolution setting
            parent.spawn(TextBundle::from_section(
                "Resolution",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
            
            // Resolution dropdown (placeholder)
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(40.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "1920 x 1080",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            
            // Fullscreen setting
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Fullscreen",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                    
                    // Checkbox (placeholder)
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                margin: UiRect::new(Val::Px(20.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "✓",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
            
            // Graphics quality setting
            parent.spawn(TextBundle::from_section(
                "Graphics Quality",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
            
            // Quality dropdown (placeholder)
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(40.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "High",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            
            // UI Scale setting
            parent.spawn(TextBundle::from_section(
                "UI Scale",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
            
            // UI Scale slider (placeholder)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(20.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(75.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.4, 0.4, 0.8)),
                            ..default()
                        },
                        SettingsSlider {
                            setting_type: SliderType::UiScale,
                            min: 0.5,
                            max: 2.0,
                            current: 1.0,
                        },
                    ));
                });
        });
}

/// Handle settings menu button interactions
fn handle_settings_buttons(
    mut button_query: Query<
        (&Interaction, &SettingsButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_close_settings: EventWriter<CloseSettingsEvent>,
    mut settings_state: ResMut<SettingsMenuState>,
) {
    for (interaction, button_type, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    SettingsButton::VideoTab => {
                        settings_state.active_tab = SettingsTab::Video;
                        println!("Video tab selected");
                    }
                    SettingsButton::AudioTab => {
                        settings_state.active_tab = SettingsTab::Audio;
                        println!("Audio tab selected");
                    }
                    SettingsButton::ControlsTab => {
                        settings_state.active_tab = SettingsTab::Controls;
                        println!("Controls tab selected");
                    }
                    SettingsButton::GameplayTab => {
                        settings_state.active_tab = SettingsTab::Gameplay;
                        println!("Gameplay tab selected");
                    }
                    SettingsButton::InterfaceTab => {
                        settings_state.active_tab = SettingsTab::Interface;
                        println!("Interface tab selected");
                    }
                    SettingsButton::Apply => {
                        println!("Apply settings");
                        // Apply settings logic would go here
                    }
                    SettingsButton::Reset => {
                        println!("Reset settings");
                        // Reset settings logic would go here
                    }
                    SettingsButton::Back => {
                        println!("Back to main menu");
                        ev_close_settings.send(CloseSettingsEvent);
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
