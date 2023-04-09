//! Config code that the user needs to edit

mod bar;
mod colorscheme;
mod consts;
mod keys;
mod window;

pub use bar::BarConfig;
pub use colorscheme::ColorSchemeConfig;
pub use consts::SCREEN_LOCK_TOOL;
pub use keys::KeyBindingConfig;
pub use window::WindowConfig;
