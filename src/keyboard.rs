use anyhow::{anyhow, Result};
use log::debug;
use std::process::Command;

pub(crate) trait Keyboard: Clone + 'static {
    fn turn_on(&self) -> Result<()>;

    fn turn_off(&self) -> Result<()>;
}

#[derive(Debug, Default, Clone)]
pub(crate) struct PcKeyboard {
    device_id: String,
}

impl PcKeyboard {
    pub(crate) fn new() -> Result<Self> {
        let device_id = String::from_utf8(
            Command::new("xinput")
                .arg("list")
                .arg("--id-only")
                .arg("AT Translated Set 2 keyboard")
                .output()?
                .stdout,
        )?
        .trim()
        .into();
        debug!("found device id: {}", device_id);
        Ok(Self { device_id })
    }
}

impl Keyboard for PcKeyboard {
    fn turn_on(&self) -> Result<()> {
        Command::new("xinput")
            .arg("reattach")
            .arg(&self.device_id)
            .arg("3")
            .spawn()?;

        Ok(())
    }

    fn turn_off(&self) -> Result<()> {
        Command::new("xinput")
            .arg("float")
            .arg(&self.device_id)
            .spawn()?;
        Ok(())
    }
}

pub(crate) fn check_xinput() -> Result<()> {
    match Command::new("xinput").arg("--version").status() {
        Ok(status) if status.success() => Ok(()),
        _ => Err(anyhow!("Looks like 'xinput' is not available. Please ensure it's working by running 'xinput --version'")),
    }
}
