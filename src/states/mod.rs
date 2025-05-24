//! Game state management

pub mod game_state;
pub mod loading;
pub mod main_menu;
pub mod gameplay;
pub mod game_over;

// Re-export commonly used items
pub use game_state::GameState;
pub use game_state::GameStatePlugin;
pub use loading::LoadingPlugin;
pub use main_menu::MainMenuPlugin;
pub use gameplay::GameplayPlugin;
pub use game_over::GameOverPlugin;
