use bevy::prelude::*;

/// Represents the different states of the game
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// The game is loading assets
    #[default]
    Loading,
    /// The main menu is being shown
    MainMenu,
    /// The game is being played
    Gameplay,
    /// The game is paused
    Paused,
    /// The game is over
    GameOver,
    /// The player has won
    Victory,
}

/// Plugin for managing the game state
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}
