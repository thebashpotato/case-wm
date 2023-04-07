//! Wraps penrose functionality into a single struct
//! for easy execution.

use crate::{bar::CaseWindowManagerStatusBar, config::KeyBindingConfig, layout::Layout};
use penrose::{
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    x11rb::RustConn,
    Result as PenroseResult,
};
use std::collections::HashMap;

/// Wraps the penrose rust connection alias type.
#[derive(Debug)]
pub struct CaseWindowManager {
    /// Internal connection to X server
    wm: WindowManager<RustConn>,
    /// Status bar loaded with users configurations
    status_bar: CaseWindowManagerStatusBar,
}

impl CaseWindowManager {
    /// Creates the case window manager instance,
    /// parses keybindings and sets up configuration and layouts.
    pub fn setup() -> PenroseResult<Self> {
        let conn = RustConn::new()?;
        let key_binding_config = KeyBindingConfig::new();
        let parsed_key_bindings =
            parse_keybindings_with_xmodmap(key_binding_config.key_bindings())?;
        let layouts = Layout::new();
        let config = Config {
            default_layouts: layouts.layouts(),
            ..Config::default()
        };
        let wm = WindowManager::new(config, parsed_key_bindings, HashMap::new(), conn)?;
        Ok(Self {
            wm,
            status_bar: CaseWindowManagerStatusBar::new(),
        })
    }

    /// Start the WindowManager and run it until told to exit.
    ///
    /// Any provided startup hooks will be run after setting signal handlers and grabbing
    /// key / mouse bindings from the X server. Any set up you need to do should be run
    /// explicitly before calling this method or as part of a startup hook.
    pub fn run(mut self) -> anyhow::Result<()> {
        let bar = self.status_bar.build_status_bar()?;
        self.wm = bar.add_to(self.wm);
        self.wm.run()?;
        Ok(())
    }
}
