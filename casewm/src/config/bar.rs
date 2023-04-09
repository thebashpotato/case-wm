//! Status Bar configuration code.

use super::consts::{
    BAR_FONT, BAR_FONT_POINT_SIZE, BAR_HEIGHT_PIXELS, BAR_MAX_ACTIVE_WINDOW_CHARS, BAR_PADDING,
    BAR_POSITION,
};
use penrose_ui::Position;

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
        // TODO: Write a font loader that verifies the users font,
        // if it doesn't exist on the system it should load a default
        // fallback font.
        Self {
            bar_height: BAR_HEIGHT_PIXELS,
            font: BAR_FONT.to_owned(),
            font_point_size: BAR_FONT_POINT_SIZE,
            padding: BAR_PADDING,
            max_active_window_chars: BAR_MAX_ACTIVE_WINDOW_CHARS,
            position: BAR_POSITION,
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
