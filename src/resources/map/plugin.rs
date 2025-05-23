use bevy::prelude::*;
use crate::resources::map_data::generate_map;

// A marker resource to indicate the map has been initialized
#[derive(Resource)]
pub struct MapInitialized(pub bool);

// Basic map plugin for Strategy Forge
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}

// Setup map system that's referenced elsewhere in the codebase
pub fn setup_map(mut commands: Commands) {
    info!("Setting up game map...");
    
    // Create the map
    let game_map = generate_map();
    
    // Insert map resources
    commands.insert_resource(game_map);
    commands.insert_resource(MapInitialized(true));
    
    info!("Map setup complete");
}
