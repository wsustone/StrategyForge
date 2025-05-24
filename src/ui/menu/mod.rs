//! Menu system for StrategyForge
//!
//! This module contains the implementation of the game's menu system,
//! including the main menu, settings menu, and in-game menus.

mod main_menu;
mod settings_menu;
mod campaign_menu;
mod skirmish_menu;
mod faction_menu;
mod profile_menu;
mod extras_menu;
mod pause_menu;
mod menu_systems;

pub use main_menu::MainMenuPlugin;
pub use settings_menu::{SettingsMenuPlugin, OpenSettingsEvent, CloseSettingsEvent};
pub use campaign_menu::{CampaignMenuPlugin, OpenCampaignMenuEvent, CloseCampaignMenuEvent};
pub use skirmish_menu::{SkirmishMenuPlugin, OpenSkirmishMenuEvent, CloseSkirmishMenuEvent};
pub use faction_menu::{FactionMenuPlugin, OpenFactionMenuEvent, CloseFactionMenuEvent};
pub use profile_menu::{ProfileMenuPlugin, OpenProfileMenuEvent, CloseProfileMenuEvent};
pub use extras_menu::{ExtrasMenuPlugin, OpenExtrasMenuEvent, CloseExtrasMenuEvent};
pub use pause_menu::PauseMenuPlugin;
pub use menu_systems::MenuInteractionPlugin;

use bevy::prelude::*;

/// Plugin that adds all menu functionality to the game
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin)
           .add_plugins(SettingsMenuPlugin)
           .add_plugins(CampaignMenuPlugin)
           .add_plugins(SkirmishMenuPlugin)
           .add_plugins(FactionMenuPlugin)
           .add_plugins(ProfileMenuPlugin)
           .add_plugins(ExtrasMenuPlugin)
           .add_plugins(PauseMenuPlugin)
           .add_plugins(MenuInteractionPlugin);
    }
}

/// Common UI components and styles used across menus
pub mod components {
    use bevy::prelude::*;
    use crate::utils::font_loader::get_font_handle;

    /// Marker component for menu UI elements
    #[derive(Component)]
    pub struct MenuUI;
    
    /// Standard menu panel dimensions
    pub struct MenuPanelDimensions {
        pub width: f32,
        pub height: f32,
        pub padding: f32,
        pub spacing: f32,
    }
    
    impl Default for MenuPanelDimensions {
        fn default() -> Self {
            Self {
                width: 600.0,
                height: 500.0,
                padding: 20.0,
                spacing: 15.0,
            }
        }
    }
    
    /// Standard menu text styles
    pub struct MenuTextStyles {
        pub title_size: f32,
        pub subtitle_size: f32,
        pub button_text_size: f32,
        pub label_size: f32,
    }
    
    impl Default for MenuTextStyles {
        fn default() -> Self {
            Self {
                title_size: 40.0,
                subtitle_size: 30.0,
                button_text_size: 24.0,
                label_size: 20.0,
            }
        }
    }

    /// Standard button colors
    pub struct ButtonColors {
        pub normal: Color,
        pub hovered: Color,
        pub pressed: Color,
        pub disabled: Color,
    }

    impl Default for ButtonColors {
        fn default() -> Self {
            Self {
                normal: Color::srgb(0.15, 0.15, 0.35),
                hovered: Color::srgb(0.25, 0.25, 0.45),
                pressed: Color::srgb(0.35, 0.35, 0.55),
                disabled: Color::srgb(0.1, 0.1, 0.2),
            }
        }
    }

    /// Create a standard menu button
    pub fn create_button(
        parent: &mut ChildBuilder,
        text: &str,
        button_type: impl Component,
        asset_server: &Res<AssetServer>,
        width: f32,
        height: f32,
    ) {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(width),
                        height: Val::Px(height),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(ButtonColors::default().normal),
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

    /// Create a standard menu panel
    pub fn create_panel(
        parent: &mut ChildBuilder,
        width: Val,
        height: Val,
        background_color: Color,
    ) -> Entity {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width,
                    height,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: BackgroundColor(background_color),
                ..default()
            })
            .id()
    }

    /// Create a standard menu title
    pub fn create_title(
        parent: &mut ChildBuilder,
        text: &str,
        asset_server: &Res<AssetServer>,
        font_size: f32,
    ) {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: get_font_handle(asset_server),
                font_size,
                color: Color::WHITE,
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        }));
    }
    
    /// Create a slider with label
    pub fn create_slider(
        parent: &mut ChildBuilder,
        label: &str,
        min: f32,
        max: f32,
        initial: f32,
        asset_server: &Res<AssetServer>,
        slider_type: impl Component,
    ) -> Entity {
        // Container for the slider and label
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Label
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: get_font_handle(asset_server),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
            
            // Slider container
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(20.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                border_color: BorderColor(Color::WHITE),
                background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
                ..default()
            })
            .with_children(|parent| {
                // Slider handle
                let normalized = (initial - min) / (max - min);
                let handle_position = normalized.clamp(0.0, 1.0) * 200.0;
                
                parent.spawn((NodeBundle {
                    style: Style {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(handle_position - 10.0), // Center the handle
                        ..default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..default()
                },
                slider_type,
                SliderHandle {
                    min,
                    max,
                    value: initial,
                    dragging: false,
                }));
            });
            
            // Value display
            parent.spawn(TextBundle::from_section(
                format!("{:.1}", initial),
                TextStyle {
                    font: get_font_handle(asset_server),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));
        })
        .id()
    }
    
    /// Slider handle component
    #[derive(Component)]
    pub struct SliderHandle {
        pub min: f32,
        pub max: f32,
        pub value: f32,
        pub dragging: bool,
    }
    
    /// Create a dropdown menu
    pub fn create_dropdown<T: Component + Clone>(
        parent: &mut ChildBuilder,
        label: &str,
        options: Vec<String>,
        selected_index: usize,
        asset_server: &Res<AssetServer>,
        dropdown_type: T,
    ) -> Entity {
        // Container for the dropdown
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Label
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: get_font_handle(asset_server),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
            
            // Dropdown button
            parent.spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(2.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                border_color: BorderColor(Color::WHITE),
                background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.35)),
                ..default()
            },
            dropdown_type.clone(),
            DropdownState {
                options: options.clone(),
                selected_index,
                is_open: false,
            }))
            .with_children(|parent| {
                // Selected option text
                parent.spawn(TextBundle::from_section(
                    options.get(selected_index).cloned().unwrap_or_default(),
                    TextStyle {
                        font: get_font_handle(asset_server),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ));
                
                // Dropdown arrow
                parent.spawn(TextBundle::from_section(
                    "▼",
                    TextStyle {
                        font: get_font_handle(asset_server),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ));
            });
        })
        .id()
    }
    
    /// Dropdown state component
    #[derive(Component)]
    pub struct DropdownState {
        pub options: Vec<String>,
        pub selected_index: usize,
        pub is_open: bool,
    }
}
