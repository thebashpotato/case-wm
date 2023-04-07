//! Window gaps and overlap configuration

/// Window configuration for gaps etc.
#[derive(Debug)]
pub struct WindowConfig {
    /// Height of status bar. A window needs to know so it
    /// doesn't overlap the status bar.
    bar_height_px: u32,
    /// Number of pixels for inner window gaps
    inner_px: u32,
    /// Number of pixels for outer window gaps
    outer_px: u32,
    /// The max amount of screen real estate the main window will take
    max_main: u32,
    /// Ratio split between main and side windows
    ratio: f32,
    /// The ratio of pixels to resize all windows
    ratio_step: f32,
}

impl WindowConfig {
    /// Builds the window config object.
    pub const fn new() -> Self {
        Self {
            bar_height_px: super::bar::BAR_HEIGHT_PIXELS,
            inner_px: 5,
            outer_px: 5,
            max_main: 1,
            ratio: 0.6,
            ratio_step: 0.1,
        }
    }

    /// Getter for window bar height
    pub const fn bar_height_px(&self) -> u32 {
        self.bar_height_px
    }

    /// Getter for inner window gap pixel size
    pub const fn inner_px(&self) -> u32 {
        self.inner_px
    }

    /// Getter for outer window gap pixel size
    pub const fn outer_px(&self) -> u32 {
        self.outer_px
    }

    /// Getter for max main
    pub const fn max_main(&self) -> u32 {
        self.max_main
    }

    /// Getter for ratio value
    pub const fn ratio(&self) -> f32 {
        self.ratio
    }

    /// Getter for ratio step value
    pub const fn ratio_step(&self) -> f32 {
        self.ratio_step
    }
}
