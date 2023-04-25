//! Key binding configuration code.

use super::consts::{SCREEN_LOCK_TOOL, SCREEN_SHOT_TOOL, TERMINAL, WINDOW_FLOATING_DELTA};
use super::get_dmenu_config;
use crate::actions::session_menu;
use penrose::{
    builtin::{
        actions::{
            exit,
            floating::{float_focused, reposition, resize, sink_all, sink_focused},
            log_current_state, modify_with, send_layout_message, spawn,
        },
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::bindings::KeyEventHandler,
    extensions::actions::launch_dmenu,
    map,
    pure::StackSet,
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
    #[allow(clippy::integer_arithmetic)]
    pub fn new() -> Self {
        let mut key_bindings: HashMap<String, Box<dyn KeyEventHandler<RustConn>>> = map! {
            map_keys: |k: &str| k.to_owned();

            // Windows
            "M-j" => modify_with(StackSet::focus_down),
            "M-k" => modify_with(StackSet::focus_up),
            "M-S-j" => modify_with(StackSet::swap_down),
            "M-S-k" => modify_with(StackSet::swap_up),
            "M-q" => modify_with(StackSet::kill_focused),

            // Workspaces, moves workspaces to a different screen.
            "M-Tab" => modify_with(StackSet::toggle_tag),
            "M-bracketright" => modify_with(StackSet::next_screen),
            "M-bracketleft" => modify_with(StackSet::previous_screen),
            "M-S-bracketright" => modify_with(StackSet::drag_workspace_forward),
            "M-S-bracketleft" => modify_with(StackSet::drag_workspace_backward),

            // Layouts
            "M-grave" => modify_with(StackSet::next_layout),
            "M-S-grave" => modify_with(StackSet::previous_layout),
            "M-Up" => send_layout_message(|| IncMain(1)),
            "M-Down" => send_layout_message(|| IncMain(-1)),
            "M-Right" => send_layout_message(|| ExpandMain),
            "M-Left" => send_layout_message(|| ShrinkMain),

            // Launchers
            "M-A-s" => spawn(SCREEN_SHOT_TOOL),
            "M-p" => launch_dmenu(get_dmenu_config(0, true)),
            "M-Return" => spawn(TERMINAL),
            "M-c" => spawn("alacritty -e calcurse"),
            //"M-slash" => Box::new(toggle_scratch),

            // Session management
            "M-A-l" => spawn(SCREEN_LOCK_TOOL),
            "M-A-space" => session_menu(get_dmenu_config(10, false)),
            "M-A-Escape" => exit(),

            // FLoating management
            "M-C-f" => float_focused(),
            "M-C-s" => sink_focused(),
            "M-C-S-s" => sink_all(),

            // Floating resize
            "M-C-Right" => resize(WINDOW_FLOATING_DELTA, 0),
            "M-C-Left" => resize(-WINDOW_FLOATING_DELTA, 0),
            "M-C-Up" => resize(0, -WINDOW_FLOATING_DELTA),
            "M-C-Down" => resize(0, WINDOW_FLOATING_DELTA),

            // Floating position
            "M-C-l" => reposition(WINDOW_FLOATING_DELTA, 0),
            "M-C-h" => reposition(-WINDOW_FLOATING_DELTA, 0),
            "M-C-k" => reposition(0, -WINDOW_FLOATING_DELTA),
            "M-C-j" => reposition(0, WINDOW_FLOATING_DELTA),

            //TODO: Load this dependant on debug cli flag
            // Debugging
            //"M-A-t" => set_tracing_filter(handle)
            "M-S-s" => log_current_state(),
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
