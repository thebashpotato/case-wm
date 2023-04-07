//! Config code that the user needs to edit
use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::bindings::KeyEventHandler,
    map,
    x11rb::RustConn,
};
use penrose_ui::bar::Position;
use std::collections::HashMap;

/// Window manger colorscheme
#[derive(Debug)]
pub struct ColorScheme {
    /// color
    black: u32,
    /// color
    white: u32,
    /// color
    grey: u32,
    /// color
    blue: u32,
}

impl ColorScheme {
    /// Construct the colorscheme object
    pub const fn new() -> Self {
        Self {
            black: 0x282828ff,
            white: 0xebdbb2ff,
            grey: 0x3c3836ff,
            blue: 0x458588ff,
        }
    }

    /// Getter for black
    pub const fn black(&self) -> u32 {
        self.black
    }

    /// Getter for white
    pub const fn white(&self) -> u32 {
        self.white
    }

    /// Getter for grey
    pub const fn grey(&self) -> u32 {
        self.grey
    }

    /// Getter for blue
    pub const fn blue(&self) -> u32 {
        self.blue
    }
}

/// Status bar configuration
#[derive(Debug)]
pub struct BarConfig {
    /// Height of the bar
    bar_height_pixel: u32,
    /// Active font
    font: String,
    /// Window characters
    max_active_window_chars: usize,
    /// Position of bar
    position: Position,
}

impl BarConfig {
    /// Builds the object
    pub fn new() -> Self {
        // TODO: Need to check for users font, and if not installed,
        // use fallback font that comes default on every linux install.
        Self {
            bar_height_pixel: 18,
            font: "JetBrainsMono Nerd Font Mono".to_owned(),
            max_active_window_chars: 50,
            position: Position::Top,
        }
    }

    /// Getter for bar_height_pixel
    pub const fn bar_height_pixel(&self) -> u32 {
        self.bar_height_pixel
    }

    /// Getter for the font
    pub fn font(&self) -> &str {
        self.font.as_str()
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

/// Configure all key bindings and shell programs that are launched via
/// key bindings.
#[derive(Debug)]
pub struct KeyBindingConfig {
    /// Holds the Key binding hashmap
    key_bindings: HashMap<String, Box<dyn KeyEventHandler<RustConn>>>,
}

impl KeyBindingConfig {
    /// Key bindings and modifiers are defined here.
    /// Modifer key is Super.
    pub fn new() -> Self {
        let mut key_bindings: HashMap<String, Box<dyn KeyEventHandler<RustConn>>> = map! {
            map_keys: |k: &str| k.to_owned();

            "M-j" => modify_with(|cs| cs.focus_down()),
            "M-k" => modify_with(|cs| cs.focus_up()),
            "M-S-j" => modify_with(|cs| cs.swap_down()),
            "M-S-k" => modify_with(|cs| cs.swap_up()),
            "M-q" => modify_with(|cs| cs.kill_focused()),
            "M-Tab" => modify_with(|cs| cs.toggle_tag()),
            "M-bracketright" => modify_with(|cs| cs.next_screen()),
            "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
            "M-grave" => modify_with(|cs| cs.next_layout()),
            "M-S-grave" => modify_with(|cs| cs.previous_layout()),
            "M-S-Up" => send_layout_message(|| IncMain(1)),
            "M-S-Down" => send_layout_message(|| IncMain(-1)),
            "M-S-Right" => send_layout_message(|| ExpandMain),
            "M-S-Left" => send_layout_message(|| ShrinkMain),
            "M-p" => spawn("dmenu_run"),
            "M-Return" => spawn("alacritty"),
            "M-A-Escape" => exit(),
        };

        for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
            key_bindings.extend([
                (
                    format!("M-{tag}"),
                    modify_with(move |client_set| client_set.focus_tag(tag)),
                ),
                (
                    format!("M-S-{tag}"),
                    modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
                ),
            ]);
        }
        Self { key_bindings }
    }

    /// Getter for key binding map
    pub fn key_bindings(self) -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
        self.key_bindings
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use super::KeyBindingConfig;
    use penrose::core::bindings::parse_keybindings_with_xmodmap;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let key_binding_config = KeyBindingConfig::new();
        let res = parse_keybindings_with_xmodmap(key_binding_config.key_bindings());
        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
