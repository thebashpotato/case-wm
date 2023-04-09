//! Window gaps and overlap configuration

use super::consts::{
    BAR_HEIGHT_PIXELS, WINDOW_GAP_INNER_PX, WINDOW_GAP_OUTER_PX, WINDOW_MAX_MAIN,
    WINDOW_RATIO_STEP, WINDOW_SPLIT_RATIO,
};

/// Window configuration for gaps etc.
#[derive(Debug)]
pub struct WindowConfig {
    /// Height of status bar. A window needs to know so it
    /// doesn't overlap the status bar.
    bar_height_px: u32,
    /// number of pixels for inner window gaps
    inner_px: u32,
    /// number of pixels for outer window gaps
    outer_px: u32,
    /// the max amount of screen real estate the main window will take
    max_main: u32,
    /// ratio split between main and side windows
    ratio: f32,
    /// the ratio of pixels to resize all windows
    ratio_step: f32,
}

impl WindowConfig {
    /// Builds the window config object.
    pub const fn new() -> Self {
        Self {
            bar_height_px: BAR_HEIGHT_PIXELS,
            inner_px: WINDOW_GAP_INNER_PX,
            outer_px: WINDOW_GAP_OUTER_PX,
            max_main: WINDOW_MAX_MAIN,
            ratio: WINDOW_SPLIT_RATIO,
            ratio_step: WINDOW_RATIO_STEP,
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
