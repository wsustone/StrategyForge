use bevy::prelude::*;
use crate::utils::font_loader::GameFonts;

/// Plugin that handles font initialization and management
pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        println!("FontPlugin::build - Adding setup_fonts system");
        app.add_systems(Startup, setup_fonts);
    }
}

/// Initialize font resources during startup
fn setup_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("setup_fonts - Starting font initialization");
    
    // Create and register the GameFonts resource
    println!("setup_fonts - Creating GameFonts resource");
    let game_fonts = GameFonts::new(&asset_server);
    
    println!("setup_fonts - Inserting GameFonts resource");
    commands.insert_resource(game_fonts);
    
    println!("setup_fonts - Font initialization complete");
}
