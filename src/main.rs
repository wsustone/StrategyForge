mod components;
mod systems;
mod resources;
mod entities;
mod states;
mod utils;

use bevy::prelude::*;
use states::loading::LoadingPlugin;
use states::main_menu::MainMenuPlugin;
use states::gameplay::GameplayPlugin;
use states::game_over::GameOverPlugin;
use systems::camera::CameraPlugin;
// Import MapPlugin directly from its source location
use crate::resources::map::plugin::MapPlugin;
// Temporarily commented out for debugging
// use crate::utils::FontPlugin;
// use crate::components::UnitLabelPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    MainMenu,
    Gameplay,
    GameOver,
}

fn main() {
    App::new()
        // Add default Bevy plugins
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Strategy Forge".into(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            },
        ))
        // Add game state management
        .init_state::<GameState>()
        
        // Add our custom plugins
        .add_plugins((
            LoadingPlugin,
            MainMenuPlugin,
            GameplayPlugin,
            GameOverPlugin,
            CameraPlugin,
            MapPlugin,
            // Temporarily disable new plugins for debugging
            // FontPlugin,
            // UnitLabelPlugin,
        ))
        
        // Run the game
        .run();
}
