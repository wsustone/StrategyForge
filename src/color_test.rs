use bevy::prelude::*;

// This function demonstrates the correct usage of colors in Bevy 0.14
pub fn test_colors() {
    // For sprite colors
    let sprite_color = Color::srgba(0.5, 0.5, 0.5, 1.0);
    
    // For UI background colors
    let background = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
    
    // For UI border colors
    let border = BorderColor(Color::srgb(0.3, 0.3, 0.3));
    
    // For text colors in TextStyle
    let text_style = TextStyle {
        font_size: 20.0,
        color: Color::srgba(0.9, 0.9, 0.9, 1.0),
        ..default()
    };
    
    println!("Color test complete!");
}
