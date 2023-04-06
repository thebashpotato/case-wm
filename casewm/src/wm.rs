//! Wraps penrose functionality into a single struct
//! for easy execution.

use crate::config::raw_key_bindings;
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
}

impl CaseWindowManager {
    /// Create the case window manager instance.
    pub fn new() -> PenroseResult<Self> {
        let conn = RustConn::new()?;
        let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
        let wm = WindowManager::new(Config::default(), key_bindings, HashMap::new(), conn)?;
        Ok(Self { wm })
    }

    /// Start the WindowManager and run it until told to exit.
    ///
    /// Any provided startup hooks will be run after setting signal handlers and grabbing
    /// key / mouse bindings from the X server. Any set up you need to do should be run
    /// explicitly before calling this method or as part of a startup hook.
    pub fn run(self) -> PenroseResult<()> {
        self.wm.run()
    }
}
