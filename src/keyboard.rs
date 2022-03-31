use anyhow::{anyhow, Result};
use cmd_lib::{run_cmd, run_fun};
use log::debug;

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
        let device_id = device_id.trim().to_string();
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
    match run_cmd!(xinput --version) {
        Ok(()) => Ok(()),
        _ => Err(anyhow!("Looks like 'xinput' is not available. Please ensure it's working by running 'xinput --version'")),
    }
}
