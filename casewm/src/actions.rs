//! Custom actions

use crate::config::SCREEN_LOCK_TOOL;
use penrose::{
    builtin::actions::key_handler,
    core::bindings::KeyEventHandler,
    extensions::util::dmenu::{DMenu, DMenuConfig, MenuMatch},
    util::spawn,
    x11rb::RustConn,
};
use std::process::exit;

/// Dmenu based power menu for common actions
#[allow(clippy::exit, clippy::unimplemented)]
pub fn session_menu(config: DMenuConfig) -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(move |state, _| {
        let screen_index = state.client_set.current_screen().index();
        let dmenu = DMenu::new(&config, screen_index);

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
