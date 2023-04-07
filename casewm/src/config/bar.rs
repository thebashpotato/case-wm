//! Status Bar configuration code.

use penrose_ui::Position;

/// Shared between window config and this config.
pub const BAR_HEIGHT_PIXELS: u32 = 25;

/// Holds all variables that are needed to render
/// the built in status bar.
#[derive(Debug)]
pub struct BarConfig {
    /// Height of the bar
    bar_height: u32,
    /// Active font
    font: String,
    /// Size of font
    font_point_size: i32,
    /// Padding around the bar text, x and y
    padding: (f64, f64),
    /// Window characters
    max_active_window_chars: usize,
    /// Position of bar
    position: Position,
}

impl BarConfig {
    /// User can add their custom values for the bar here.
    pub fn new() -> Self {
        // TODO: Need to check for users font, and if not installed,
        // use fallback font that comes default on every linux install.
        Self {
            bar_height: BAR_HEIGHT_PIXELS,
            font: "JetBrainsMono Nerd Font Mono".to_owned(),
            font_point_size: 12,
            padding: (2.0, 2.0),
            max_active_window_chars: 50,
            position: Position::Top,
        }
    }

    /// Getter for the height of the bar
    pub const fn bar_height(&self) -> u32 {
        self.bar_height
    }

    /// Getter for the font
    pub fn font(&self) -> &str {
        self.font.as_str()
    }

    /// Getter for the font
    pub const fn font_point_size(&self) -> i32 {
        self.font_point_size
    }

    /// Getter for bar x and y padding
    pub const fn padding(&self) -> (f64, f64) {
        self.padding
    }

    /// Getter for the maximum amount of window characters
    pub const fn max_active_window_chars(&self) -> usize {
        self.max_active_window_chars
    }

    /// Getter for bar position
    pub const fn position(&self) -> Position {
        self.position
    }
}
