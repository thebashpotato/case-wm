//! The case window manager.

use casewm::CaseWindowManager;

fn main() -> anyhow::Result<()> {
    let casewm = CaseWindowManager::setup()?;
    casewm.run()?;

    Ok(())
}
