//! Wraps penrose functionality into a single struct for easy execution.

use crate::{config::KeyBindingConfig, layout::Layout, status_bar::CaseWmStatusBar};
use penrose::{
    core::{bindings::parse_keybindings_with_xmodmap, Config, WindowManager},
    extensions::hooks::{add_ewmh_hooks, SpawnOnStartup},
    x11rb::RustConn,
    Result as PenroseResult,
};
use std::collections::HashMap;
use tracing_subscriber::prelude::*;

/// Wraps the penrose rust connection alias type.
#[derive(Debug)]
pub struct CaseWindowManager {
    /// Internal connection to X server
    wm: WindowManager<RustConn>,
    /// Status bar loaded with users configurations
    status_bar: CaseWmStatusBar,
}

impl CaseWindowManager {
    /// Creates the case window manager instance,
    /// parses keybindings and sets up configuration and layouts.
    ///
    /// # Errors
    /// * `penrose::Result`
    pub fn setup() -> PenroseResult<Self> {
        tracing_subscriber::fmt()
            .with_env_filter("info")
            .finish()
            .init();
        let conn = RustConn::new()?;
        let key_binding_config = KeyBindingConfig::new();
        let status_bar = CaseWmStatusBar::new();
        let layouts = Layout::new();
        let parsed_key_bindings =
            parse_keybindings_with_xmodmap(key_binding_config.key_bindings())?;

        let config = add_ewmh_hooks(Config {
            default_layouts: layouts.layouts(),
            startup_hook: Some(SpawnOnStartup::boxed("/usr/local/bin/casewm_startup.sh")),
            ..Config::default()
        });
        let wm = WindowManager::new(config, parsed_key_bindings, HashMap::new(), conn)?;
        Ok(Self { wm, status_bar })
    }

    /// Start the window manager and run it until told to exit.
    ///
    /// Any provided startup hooks will be run after setting signal handlers and grabbing
    /// key / mouse bindings from the X server. Any set up you need to do should be run
    /// explicitly before calling this method or as part of a startup hook.
    ///
    /// # Errors
    /// * `penrose_ui::Result`
    /// * `penrose::Result`
    pub fn run(mut self) -> anyhow::Result<()> {
        let bar = self.status_bar.build_status_bar()?;
        self.wm = bar.add_to(self.wm);
        self.wm.run()?;
        Ok(())
    }
}
