//! MVP window manager

use casewm::CaseWindowManager;
use penrose::Result as PenroseResult;
use tracing_subscriber::prelude::*;

fn main() -> PenroseResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let case_wm = CaseWindowManager::new()?;
    case_wm.run()
}
