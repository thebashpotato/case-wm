//! Config code that the user needs to edit

mod bar;
mod colorscheme;
mod consts;
mod keys;
mod window;

pub use bar::BarConfig;
pub use colorscheme::ColorSchemeConfig;
pub use consts::{BAR_FONT, SCREEN_LOCK_TOOL};
pub use keys::KeyBindingConfig;
pub use window::WindowConfig;

use penrose::extensions::util::dmenu::{DMenuConfig, DMenuKind};

/// Getter function for case's dmenu configuration
///
/// # Arguments
///  * `n_lines` the number of lines to show, 0 means the content will
///     be displayed horizontally.
pub fn get_dmenu_config(n_lines: u8) -> DMenuConfig {
    let colorscheme = ColorSchemeConfig::new();
    DMenuConfig {
        n_lines,
        custom_prompt: Some(" ".to_owned()),
        kind: DMenuKind::Rust,
        custom_font: Some(BAR_FONT.to_owned()),
        bg_color: colorscheme.background(),
        fg_color: colorscheme.foreground(),
        selected_color: colorscheme.primary(),
        ..Default::default()
    }
}
