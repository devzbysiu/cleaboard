use anyhow::{anyhow, Result};
use cmd_lib::{run_cmd, run_fun};
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
        let device_id = run_fun!(xinput list --id-only "AT Translated Set 2 keyboard")?;
        debug!("found device id: {}", device_id);
        Ok(Self { device_id })
    }
}

impl Keyboard for PcKeyboard {
    fn turn_on(&self) -> Result<()> {
        let device_id = &self.device_id;
        run_cmd!(xinput reattach $device_id 3)?;
        Ok(())
    }

    fn turn_off(&self) -> Result<()> {
        let device_id = &self.device_id;
        run_cmd!(xinput float $device_id)?;
        Ok(())
    }
}

pub(crate) fn check_xinput() -> Result<()> {
    match Command::new("xinput").arg("--version").status() {
        Ok(status) if status.success() => Ok(()),
        _ => Err(anyhow!("Looks like 'xinput' is not available. Please ensure it's working by running 'xinput --version'")),
    }
}
