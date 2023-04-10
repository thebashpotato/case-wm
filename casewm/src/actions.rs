//! Custom actions

use crate::{
    config::{BAR_FONT, SCREEN_LOCK_TOOL},
    utils::{DMenu, DMenuConfig, DMenuKind},
};
use penrose::{
    builtin::actions::key_handler, core::bindings::KeyEventHandler,
    extensions::util::dmenu::MenuMatch, util::spawn, x11rb::RustConn,
};
use std::process::exit;

/// Dmenu based power menu for common actions
#[allow(clippy::exit, clippy::unimplemented)]
pub fn session_menu() -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(|state, _| {
        let dm_config = DMenuConfig {
            custom_font: Some(BAR_FONT),
            kind: DMenuKind::Rust,
            ..DMenuConfig::default()
        };
        let screen_index = state.client_set.current_screen().index();
        let dmenu = DMenu::new(Some(" "), dm_config, screen_index);

        let options = vec!["lock", "logout", "restart-wm", "shutdown", "reboot"];
        if let Ok(MenuMatch::Line(_, choice)) = dmenu.build_menu(options) {
            match choice.as_ref() {
                "lock" => spawn(SCREEN_LOCK_TOOL),
                "logout" => spawn("pkill -fi casewm"),
                "shutdown" => spawn("sudo shutdown -h now"),
                "reboot" => spawn("sudo reboot"),
                "restart-wm" => exit(0),
                _ => unimplemented!(),
            }
        } else {
            Ok(())
        }
    })
}

/// Simply run a dmenu prompt with flags
pub fn run_dmenu() -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(|state, _| {
        let dm_config = DMenuConfig {
            custom_font: Some(BAR_FONT),
            n_lines: 0,
            kind: DMenuKind::Rust,
            ..DMenuConfig::default()
        };
        let screen_index = state.client_set.current_screen().index();
        let dmenu = DMenu::new(Some(" "), dm_config, screen_index);
        dmenu.run()
    })
}
