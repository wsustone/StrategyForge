use bevy::prelude::*;
use super::components::{SliderHandle, DropdownState};

/// Plugin for common menu interaction systems
pub struct MenuInteractionPlugin;

impl Plugin for MenuInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_slider_interaction,
            handle_dropdown_interaction,
        ));
    }
}

/// System to handle slider interactions
fn handle_slider_interaction(
    mut slider_query: Query<(&mut SliderHandle, &mut Style, &Node, &Parent), With<SliderHandle>>,
    parent_query: Query<(&Node, &GlobalTransform), Without<SliderHandle>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SliderHandle>)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    // Handle starting and stopping dragging
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                for (mut handle, _, _, _) in slider_query.iter_mut() {
                    handle.dragging = true;
                }
            }
            _ => {}
        }
    }

    if buttons.just_released(MouseButton::Left) {
        for (mut handle, _, _, _) in slider_query.iter_mut() {
            handle.dragging = false;
        }
    }

    // Handle dragging
    if let Some(cursor_position) = cursor_moved_events.read().last().map(|event| event.position) {
        for (mut handle, mut style, node, parent) in slider_query.iter_mut() {
            if handle.dragging {
                if let Ok((parent_node, parent_transform)) = parent_query.get(parent.get()) {
                    let parent_width = parent_node.size().x;
                    let parent_position = parent_transform.translation().truncate();
                    
                    // Calculate slider position
                    let left_edge = parent_position.x - parent_width / 2.0;
                    let relative_x = cursor_position.x - left_edge;
                    let normalized = (relative_x / parent_width).clamp(0.0, 1.0);
                    
                    // Update handle position
                    style.left = Val::Px(normalized * parent_width - node.size().x / 2.0);
                    
                    // Update value
                    handle.value = handle.min + normalized * (handle.max - handle.min);
                }
            }
        }
    }
}

/// System to handle dropdown interactions
fn handle_dropdown_interaction(
    mut commands: Commands,
    mut dropdown_query: Query<(Entity, &mut DropdownState, &mut BackgroundColor), (With<Button>, Changed<Interaction>)>,
    interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<Button>)>,
    dropdown_option_query: Query<(&Parent, &DropdownOptionMarker)>,
    asset_server: Res<AssetServer>,
) {
    // Handle dropdown button clicks
    for (interaction, entity) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Check if this is a dropdown button
            if let Ok((dropdown_entity, mut state, mut bg_color)) = dropdown_query.get_mut(entity) {
                // Toggle dropdown state
                state.is_open = !state.is_open;
                
                // Update background color
                if state.is_open {
                    *bg_color = BackgroundColor(Color::srgb(0.25, 0.25, 0.45));
                } else {
                    *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.35));
                }
                
                // If opening, create dropdown options
                if state.is_open {
                    commands.entity(dropdown_entity).with_children(|parent| {
                        // Options container
                        parent.spawn(NodeBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                top: Val::Px(30.0),
                                width: Val::Px(250.0),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
                            z_index: ZIndex::Global(100),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Create option buttons
                            for (i, option) in state.options.iter().enumerate() {
                                parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(30.0),
                                            padding: UiRect::all(Val::Px(5.0)),
                                            ..default()
                                        },
                                        background_color: BackgroundColor(
                                            if i == state.selected_index {
                                                Color::srgb(0.3, 0.3, 0.5)
                                            } else {
                                                Color::srgb(0.2, 0.2, 0.4)
                                            }
                                        ),
                                        ..default()
                                    },
                                    DropdownOptionMarker { index: i },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        option.clone(),
                                        TextStyle {
                                            font: crate::utils::font_loader::get_font_handle(&asset_server),
                                            font_size: 18.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });
                            }
                        });
                    });
                } else {
                    // If closing, remove dropdown options
                    commands.entity(dropdown_entity).despawn_descendants();
                }
            }
            
            // Check if this is a dropdown option
            if let Ok((parent, option_marker)) = dropdown_option_query.get(entity) {
                if let Ok((_, mut state, _)) = dropdown_query.get_mut(parent.get()) {
                    // Update selected index
                    state.selected_index = option_marker.index;
                    state.is_open = false;
                    
                    // Remove dropdown options
                    commands.entity(parent.get()).despawn_descendants();
                }
            }
        }
    }
}

/// Marker component for dropdown options
#[derive(Component)]
struct DropdownOptionMarker {
    index: usize,
}
