//! Key binding configuration code.

use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::bindings::KeyEventHandler,
    map,
    x11rb::RustConn,
};
use std::collections::HashMap;

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
