use bevy::prelude::*;
use super::components::{MenuUI, create_button, create_panel, create_title};

/// Plugin for the extras menu
pub struct ExtrasMenuPlugin;

impl Plugin for ExtrasMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenExtrasMenuEvent>()
           .add_event::<CloseExtrasMenuEvent>()
           .add_systems(Update, handle_open_extras_menu_event)
           .add_systems(Update, handle_extras_menu_buttons.run_if(resource_exists::<ExtrasMenuState>))
           .add_systems(Update, handle_close_extras_menu_event);
    }
}

/// Event to open the extras menu
#[derive(Event)]
pub struct OpenExtrasMenuEvent;

/// Event to close the extras menu
#[derive(Event)]
pub struct CloseExtrasMenuEvent;

/// State of the extras menu
#[derive(Resource)]
struct ExtrasMenuState;

/// Marker component for extras menu UI elements
#[derive(Component)]
struct ExtrasMenuUI;

/// Extras menu button types
#[derive(Component)]
enum ExtrasMenuButton {
    Artbook,
    Soundtrack,
    Credits,
    PatchNotes,
    Community,
    Back,
}

/// Handle the open extras menu event
fn handle_open_extras_menu_event(
    mut commands: Commands,
    mut ev_open_extras: EventReader<OpenExtrasMenuEvent>,
    asset_server: Res<AssetServer>,
    extras_state: Option<Res<ExtrasMenuState>>,
) {
    for _ in ev_open_extras.read() {
        // Only open extras menu if it's not already open
        if extras_state.is_none() {
            commands.insert_resource(ExtrasMenuState);
            
            // Placeholder for extras menu UI
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
                        z_index: ZIndex::Global(10),
                        ..default()
                    },
                    ExtrasMenuUI,
                    MenuUI,
                ))
                .with_children(|parent| {
                    // Extras menu panel
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(600.0),
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
                            create_title(parent, "EXTRAS", &asset_server, 40.0);
                            
                            // Artbook button
                            create_button(parent, "Artbook", ExtrasMenuButton::Artbook, &asset_server, 300.0, 50.0);
                            
                            // Soundtrack button
                            create_button(parent, "Soundtrack", ExtrasMenuButton::Soundtrack, &asset_server, 300.0, 50.0);
                            
                            // Credits button
                            create_button(parent, "Credits", ExtrasMenuButton::Credits, &asset_server, 300.0, 50.0);
                            
                            // Patch Notes button
                            create_button(parent, "Patch Notes", ExtrasMenuButton::PatchNotes, &asset_server, 300.0, 50.0);
                            
                            // Community button
                            create_button(parent, "Community", ExtrasMenuButton::Community, &asset_server, 300.0, 50.0);
                            
                            // Back button
                            create_button(parent, "Back", ExtrasMenuButton::Back, &asset_server, 300.0, 50.0);
                        });
                });
        }
    }
}

/// Handle the close extras menu event
fn handle_close_extras_menu_event(
    mut commands: Commands,
    mut ev_close_extras: EventReader<CloseExtrasMenuEvent>,
    extras_ui_query: Query<Entity, With<ExtrasMenuUI>>,
    extras_state: Option<Res<ExtrasMenuState>>,
) {
    for _ in ev_close_extras.read() {
        if extras_state.is_some() {
            // Remove extras UI
            for entity in extras_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            
            // Remove extras state
            commands.remove_resource::<ExtrasMenuState>();
        }
    }
}

/// Handle extras menu button interactions
fn handle_extras_menu_buttons(
    mut button_query: Query<
        (&Interaction, &ExtrasMenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_close_extras: EventWriter<CloseExtrasMenuEvent>,
) {
    for (interaction, button_type, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    ExtrasMenuButton::Artbook => {
                        println!("Artbook button pressed!");
                        // Artbook functionality would go here
                    }
                    ExtrasMenuButton::Soundtrack => {
                        println!("Soundtrack button pressed!");
                        // Soundtrack functionality would go here
                    }
                    ExtrasMenuButton::Credits => {
                        println!("Credits button pressed!");
                        // Credits functionality would go here
                    }
                    ExtrasMenuButton::PatchNotes => {
                        println!("Patch Notes button pressed!");
                        // Patch notes functionality would go here
                    }
                    ExtrasMenuButton::Community => {
                        println!("Community button pressed!");
                        // Community functionality would go here
                    }
                    ExtrasMenuButton::Back => {
                        println!("Back button pressed!");
                        ev_close_extras.send(CloseExtrasMenuEvent);
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
