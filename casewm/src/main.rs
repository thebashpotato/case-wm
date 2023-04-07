//! MVP window manager

use casewm::CaseWindowManager;
use tracing_subscriber::prelude::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let case_window_manager = CaseWindowManager::setup()?;
    case_window_manager.run()?;

    Ok(())
}
