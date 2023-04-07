//! Colorscheme configuration code.

use penrose::Color;

/// Window manger colorscheme
#[derive(Debug)]
pub struct ColorSchemeConfig {
    /// color
    black: u32,
    /// color
    white: u32,
    /// color
    grey: u32,
    /// color
    blue: u32,
}

impl ColorSchemeConfig {
    /// Construct the colorscheme object
    pub const fn new() -> Self {
        Self {
            black: 0x282828ff,
            white: 0xebdbb2ff,
            grey: 0x3c3836ff,
            blue: 0x458588ff,
        }
    }

    /// Getter for black.
    /// Converts from hex repr to penrose rgba repr
    pub fn black(&self) -> Color {
        self.black.into()
    }

    /// Getter for white.
    /// Converts from hex repr to penrose rgba repr
    pub fn white(&self) -> Color {
        self.white.into()
    }

    /// Getter for grey.
    /// Converts from hex repr to penrose rgba repr
    pub fn grey(&self) -> Color {
        self.grey.into()
    }

    /// Getter for blue.
    /// Converts from hex repr to penrose rgba repr
    pub fn blue(&self) -> Color {
        self.blue.into()
    }
}
