//! MVP window manager

use case_wm::CaseWindowManager;
use penrose::Result as PenroseResult;
use tracing_subscriber::prelude::*;

fn main() -> PenroseResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let cwm = CaseWindowManager::new()?;
    cwm.run()
}
