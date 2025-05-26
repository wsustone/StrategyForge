use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

/// System to handle display mode button clicks
pub fn handle_display_mode_buttons(
    mut interaction_query: Query<(
        &Interaction,
        &DisplayModeButton,
        &mut BackgroundColor,
    ), (Changed<Interaction>, With<Button>>)>,
    mode_options_query: Query<&Children, With<DisplayModeOptions>>,
    mut buttons_query: Query<&mut BackgroundColor, With<DisplayModeButton>>,
    mut change_mode_events: EventWriter<ChangeDisplayMode>,
) {
    for (interaction, display_mode, mut background_color) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Update all buttons to unselected state
            if let Ok(children) = mode_options_query.get_single() {
                for &child in children {
                    if let Ok(mut bg) = buttons_query.get_mut(child) {
                        bg.0 = Color::srgb(0.2, 0.2, 0.4).into();
                    }
                }
            }
            
            // Set the clicked button to selected state
            background_color.0 = Color::srgb(0.3, 0.6, 1.0).into();
            
            // Send event to change the display mode
            change_mode_events.send(ChangeDisplayMode(display_mode.0));
        }
    }
}

/// System to apply display mode changes
pub fn apply_display_mode_changes(
    mut change_mode_events: EventReader<ChangeDisplayMode>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    for event in change_mode_events.read() {
        if let Ok(mut window) = windows.get_single_mut() {
            window.mode = event.0;
            println!("Changed display mode to: {:?}", event.0);
        }
    }
}
