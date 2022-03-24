use anyhow::{anyhow, Result};
use std::process::Command;

pub(crate) trait Keyboard: Copy + 'static {
    fn turn_on(&self) -> Result<()>;

    fn turn_off(&self) -> Result<()>;
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct PcKeyboard;

impl Keyboard for PcKeyboard {
    fn turn_on(&self) -> Result<()> {
        Command::new("xinput")
            .arg("reattach")
            .arg("17")
            .arg("3")
            .spawn()?;

        Ok(())
    }

    fn turn_off(&self) -> Result<()> {
        Command::new("xinput").arg("float").arg("17").spawn()?;
        Ok(())
    }
}

pub(crate) fn check_xinput() -> Result<()> {
    match Command::new("xinput").arg("--version").status() {
        Ok(status) if status.success() => Ok(()),
        _ => Err(anyhow!("Looks like 'xinput' is not available. Please ensure it's working by running 'xinput --version'")),
    }
}
