//! Constant configuration variables.

use penrose_ui::Position;

/////////////////////////////////////////
//     Bar configuration              //
///////////////////////////////////////

/// Height of the status bar in pixels
pub const BAR_HEIGHT_PIXELS: u32 = 28;

/// Position of the status bar/ Top or Bottom
pub const BAR_POSITION: Position = Position::Top;

/// Active font, if font is not found monospace will be loaded as fallback.
pub const BAR_FONT: &str = "JetBrainsMono Nerd Font Mono";

/// Size of font
pub const BAR_FONT_POINT_SIZE: i32 = 15;

/// Padding around the bar text, X and Y
pub const BAR_PADDING: (f64, f64) = (10.0, 10.0);

/// Max amount of window characters that the status bar will display.
pub const BAR_MAX_ACTIVE_WINDOW_CHARS: usize = 50;

/////////////////////////////////////////
//     Window configuration           //
///////////////////////////////////////

/// The amount of pixels the resize/move a floating window
pub const WINDOW_FLOATING_DELTA: i32 = 20;

/// Number of pixels for inner window gaps
pub const WINDOW_GAP_INNER_PX: u32 = 5;

/// Number of pixels for outer window gaps
pub const WINDOW_GAP_OUTER_PX: u32 = 15;

/// The max amount of screen real estate the main window will take
pub const WINDOW_MAX_MAIN: u32 = 1;

/// Ratio split between main and side windows in the main/side layout
pub const WINDOW_SPLIT_RATIO: f32 = 0.6;

/// The ratio of pixels to resize all windows
pub const WINDOW_RATIO_STEP: f32 = 0.1;

/////////////////////////////////////////
//     Default Launch Programs        //
///////////////////////////////////////

/// Terminal
pub const TERMINAL: &str = "alacritty";

/// X Screen shot tool
pub const SCREEN_SHOT_TOOL: &str = "scrot";

/// X Screen lock tool
pub const SCREEN_LOCK_TOOL: &str = "slock";
