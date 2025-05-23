mod components;
mod systems;
mod resources;
mod entities;
mod states;
mod utils;
mod ui;
mod debug;
mod units;
mod sprites;

use bevy::prelude::*;
use states::loading::LoadingPlugin;
use states::main_menu::MainMenuPlugin;
use states::gameplay::GameplayPlugin;
use states::game_over::GameOverPlugin;
use systems::camera::CameraPlugin;
// Import MapPlugin directly from its source location
use crate::resources::map::plugin::MapPlugin;
// Import our new MovementPlugin
use crate::systems::movement::MovementPlugin;
use crate::systems::base_movement::BaseMovePlugin;
use crate::systems::production::ProductionPlugin;
use crate::components::AIPlugin;
use crate::ui::{BaseActionUIPlugin, BuildingProductionUIPlugin, BuildingSelectionUIPlugin};
use crate::debug::DebugPlugin;
use crate::components::strategic::StrategicLocationPlugin;
use crate::units::EngineerPlugin;
use crate::resources::ResourceNodePlugin;
// Import our sprite loader
use crate::sprites::SpriteLoaderPlugin;
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
        
        // Add our custom plugins individually
        .add_plugins(LoadingPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameplayPlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(BaseMovePlugin)
        .add_plugins(AIPlugin)
        .add_plugins(BaseActionUIPlugin)
        .add_plugins(BuildingProductionUIPlugin)
        .add_plugins(BuildingSelectionUIPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(StrategicLocationPlugin)
        .add_plugins(EngineerPlugin)
        .add_plugins(ResourceNodePlugin)
        .add_plugins(ProductionPlugin)
        .add_plugins(SpriteLoaderPlugin)
        // Temporarily disable new plugins for debugging
        // .add_plugins(FontPlugin)
        // .add_plugins(UnitLabelPlugin)
        
        // Run the game
        .run();
}
