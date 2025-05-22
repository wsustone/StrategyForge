use bevy::prelude::*;
use crate::GameState;
use crate::utils::font_loader::get_font_handle;

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
    Play,
    Settings,
    Quit,
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Set up camera
    commands.spawn(Camera2dBundle::default());
    
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
                background_color: Color::srgb(0.1, 0.1, 0.3).into(),
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
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            )
            .with_text_justify(JustifyText::Center));
            
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
                    // Play button
                    spawn_button(parent, "Play Game", MenuButton::Play, &asset_server);
                    
                    // Settings button
                    spawn_button(parent, "Settings", MenuButton::Settings, &asset_server);
                    
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
                background_color: Color::srgb(0.15, 0.15, 0.35).into(),
                ..default()
            },
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: get_font_handle(&asset_server),
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
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
) {
    for (interaction, button_type, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    MenuButton::Play => {
                        next_state.set(GameState::Gameplay);
                    }
                    MenuButton::Settings => {
                        // Settings functionality would go here
                        // For now we'll just print to console
                        println!("Settings button pressed!");
                    }
                    MenuButton::Quit => {
                        app_exit_events.send(AppExit::default());
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0.25, 0.25, 0.45).into();
            }
            Interaction::None => {
                *background_color = Color::srgb(0.15, 0.15, 0.35).into();
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
