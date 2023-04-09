//! Colorscheme configuration code.

use penrose::Color;

/// Window manger colorscheme
#[derive(Debug)]
pub struct ColorSchemeConfig {
    /// Background color, mostly used for bar
    background: u32,
    /// Foreground color mostly used for bar
    foreground: u32,
    /// Inactive client highlight color
    inactive: u32,
    /// Primary color that will be used for bar and window borders
    primary: u32,
}

impl ColorSchemeConfig {
    /// Construct the colorscheme object
    pub const fn new() -> Self {
        Self {
            background: 0x1d20_21ff,
            foreground: 0xa899_84ff,
            inactive: 0xebdb_b2ff,
            primary: 0x4585_88ff,
        }
    }

    /// Getter for background color
    /// Converts from hex repr to penrose rgba repr
    pub fn background(&self) -> Color {
        self.background.into()
    }

    /// Getter for foreground color
    /// Converts from hex repr to penrose rgba repr
    pub fn foreground(&self) -> Color {
        self.foreground.into()
    }

    /// Getter for inactive color
    /// Converts from hex repr to penrose rgba repr
    pub fn inactive(&self) -> Color {
        self.inactive.into()
    }

    /// Getter for primary color
    /// Converts from hex repr to penrose rgba repr
    pub fn primary(&self) -> Color {
        self.primary.into()
    }
}
