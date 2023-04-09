//! The case window manager.

use casewm::CaseWm;
use tracing_subscriber::prelude::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let casewm = CaseWm::setup()?;
    casewm.run()?;

    Ok(())
}
