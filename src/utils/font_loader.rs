use bevy::prelude::*;

/// Represents different font styles available in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Title,      // Large, bold font for titles
    Button,     // Medium font for buttons
    Body,       // Regular text font
    Small,      // Small text for detailed info
    UnitLabel,  // Font used for unit labels in-game
    Status,     // Font used for status messages
}

/// Resource to store all loaded fonts
#[derive(Resource)]
pub struct GameFonts {
    pub title: Handle<Font>,
    pub button: Handle<Font>,
    pub body: Handle<Font>,
    pub small: Handle<Font>,
    pub unit_label: Handle<Font>,
    pub status: Handle<Font>,
}

impl GameFonts {
    /// Creates a new GameFonts resource with all required fonts loaded
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        // Define our font paths
        let title_font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let button_font = asset_server.load("fonts/FiraSans-SemiBold.ttf");
        let body_font = asset_server.load("fonts/FiraSans-Regular.ttf");
        let small_font = asset_server.load("fonts/FiraSans-Light.ttf");
        let unit_label_font = asset_server.load("fonts/FiraSans-Medium.ttf");
        let status_font = asset_server.load("fonts/FiraSans-Regular.ttf");
        
        // Create our font resource
        Self {
            title: title_font,
            button: button_font,
            body: body_font,
            small: small_font,
            unit_label: unit_label_font,
            status: status_font,
        }
    }

    /// Get a font handle by its style
    pub fn get(&self, style: FontStyle) -> Handle<Font> {
        match style {
            FontStyle::Title => self.title.clone(),
            FontStyle::Button => self.button.clone(),
            FontStyle::Body => self.body.clone(),
            FontStyle::Small => self.small.clone(),
            FontStyle::UnitLabel => self.unit_label.clone(),
            FontStyle::Status => self.status.clone(),
        }
    }
}

/// Creates a TextStyle with the appropriate font and size for a given style
pub fn get_text_style(fonts: &Res<GameFonts>, style: FontStyle, color: Color) -> TextStyle {
    let font = fonts.get(style);
    let size = match style {
        FontStyle::Title => 80.0,
        FontStyle::Button => 30.0,
        FontStyle::Body => 24.0,
        FontStyle::Small => 18.0,
        FontStyle::UnitLabel => 16.0,
        FontStyle::Status => 22.0,
    };
    
    TextStyle {
        font,
        font_size: size,
        color,
    }
}

/// Legacy function for backward compatibility
pub fn get_font_handle(asset_server: &Res<AssetServer>) -> Handle<Font> {
    // Use our custom font file if it exists
    asset_server.load("fonts/FiraSans-Bold.ttf")
}
