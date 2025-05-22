use bevy::prelude::*;

pub fn get_font_handle(asset_server: &Res<AssetServer>) -> Handle<Font> {
    // Try to load the specified font, and fall back to default font if it fails
    let font_handle: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    font_handle
}

// Use this function in places where you need a font handle
// Example:
// let font = get_font_handle(&asset_server);
