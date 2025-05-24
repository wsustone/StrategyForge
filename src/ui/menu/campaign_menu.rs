use bevy::prelude::*;
use super::components::{MenuUI, create_button, create_panel, create_title};

/// Plugin for the campaign menu
pub struct CampaignMenuPlugin;

impl Plugin for CampaignMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenCampaignMenuEvent>()
           .add_event::<CloseCampaignMenuEvent>()
           .add_systems(Update, handle_open_campaign_menu_event)
           .add_systems(Update, handle_campaign_menu_buttons.run_if(resource_exists::<CampaignMenuState>))
           .add_systems(Update, handle_close_campaign_menu_event);
    }
}

/// Event to open the campaign menu
#[derive(Event)]
pub struct OpenCampaignMenuEvent;

/// Event to close the campaign menu
#[derive(Event)]
pub struct CloseCampaignMenuEvent;

/// State of the campaign menu
#[derive(Resource)]
struct CampaignMenuState;

/// Marker component for campaign menu UI elements
#[derive(Component)]
struct CampaignMenuUI;

/// Campaign menu button types
#[derive(Component)]
enum CampaignMenuButton {
    NewCampaign,
    ContinueCampaign,
    MissionSelect,
    Difficulty,
    Back,
}

/// Handle the open campaign menu event
fn handle_open_campaign_menu_event(
    mut commands: Commands,
    mut ev_open_campaign: EventReader<OpenCampaignMenuEvent>,
    asset_server: Res<AssetServer>,
    campaign_state: Option<Res<CampaignMenuState>>,
) {
    for _ in ev_open_campaign.read() {
        // Only open campaign menu if it's not already open
        if campaign_state.is_none() {
            commands.insert_resource(CampaignMenuState);
            
            // Placeholder for campaign menu UI
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
                    CampaignMenuUI,
                    MenuUI,
                ))
                .with_children(|parent| {
                    // Campaign menu panel
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
                            create_title(parent, "CAMPAIGN", &asset_server, 40.0);
                            
                            // New Campaign button
                            create_button(parent, "New Campaign", CampaignMenuButton::NewCampaign, &asset_server, 300.0, 50.0);
                            
                            // Continue Campaign button
                            create_button(parent, "Continue Campaign", CampaignMenuButton::ContinueCampaign, &asset_server, 300.0, 50.0);
                            
                            // Mission Select button
                            create_button(parent, "Mission Select", CampaignMenuButton::MissionSelect, &asset_server, 300.0, 50.0);
                            
                            // Difficulty button
                            create_button(parent, "Difficulty Settings", CampaignMenuButton::Difficulty, &asset_server, 300.0, 50.0);
                            
                            // Back button
                            create_button(parent, "Back", CampaignMenuButton::Back, &asset_server, 300.0, 50.0);
                        });
                });
        }
    }
}

/// Handle the close campaign menu event
fn handle_close_campaign_menu_event(
    mut commands: Commands,
    mut ev_close_campaign: EventReader<CloseCampaignMenuEvent>,
    campaign_ui_query: Query<Entity, With<CampaignMenuUI>>,
    campaign_state: Option<Res<CampaignMenuState>>,
) {
    for _ in ev_close_campaign.read() {
        if campaign_state.is_some() {
            // Remove campaign UI
            for entity in campaign_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            
            // Remove campaign state
            commands.remove_resource::<CampaignMenuState>();
        }
    }
}

/// Handle campaign menu button interactions
fn handle_campaign_menu_buttons(
    mut button_query: Query<
        (&Interaction, &CampaignMenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_close_campaign: EventWriter<CloseCampaignMenuEvent>,
) {
    for (interaction, button_type, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    CampaignMenuButton::NewCampaign => {
                        println!("New Campaign button pressed!");
                        // New campaign functionality would go here
                    }
                    CampaignMenuButton::ContinueCampaign => {
                        println!("Continue Campaign button pressed!");
                        // Continue campaign functionality would go here
                    }
                    CampaignMenuButton::MissionSelect => {
                        println!("Mission Select button pressed!");
                        // Mission select functionality would go here
                    }
                    CampaignMenuButton::Difficulty => {
                        println!("Difficulty Settings button pressed!");
                        // Difficulty settings functionality would go here
                    }
                    CampaignMenuButton::Back => {
                        println!("Back button pressed!");
                        ev_close_campaign.send(CloseCampaignMenuEvent);
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
