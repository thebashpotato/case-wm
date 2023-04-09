//! Custom actions

use crate::config::ColorSchemeConfig;
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
pub fn power_menu() -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(|state, _| {
        let colorscheme = ColorSchemeConfig::new();
        let dconfig = DMenuConfig {
            bg_color: colorscheme.background(),
            fg_color: colorscheme.foreground(),
            selected_color: colorscheme.primary(),
            ..DMenuConfig::default()
        };
        let options = vec!["lock", "logout", "restart-wm", "shutdown", "reboot"];
        //let menu = DMenu::new(" ", options, DMenuConfig::default());
        let menu = DMenu::new(">>> ", options, dconfig);
        let screen_index = state.client_set.current_screen().index();

        if let Ok(MenuMatch::Line(_, choice)) = menu.run(screen_index) {
            match choice.as_ref() {
                "lock" => spawn(SCREEN_LOCK_TOOL),
                "logout" => spawn("pkill -fi casewm"),
                "shutdown" => spawn("sudo shutdown -h now"),
                "reboot" => spawn("sudo reboot"),
                "restart-wm" => exit(0), // Wrapper script then handles restarting us
                _ => unimplemented!(),
            }
        } else {
            Ok(())
        }
    })
}
