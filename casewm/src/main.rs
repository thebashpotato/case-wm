//! MVP window manager

use case_wm::CaseWindowManager;
use tracing_subscriber::prelude::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let cwm = CaseWindowManager::new()?;
    cwm.run()?;

    Ok(())
}
