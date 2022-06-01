use crate::keyboard::{check_xinput, PcKeyboard};
use crate::state::State;
use crate::ui::ui_builder;

use druid::{AppLauncher, PlatformError, WindowDesc};

mod keyboard;
mod state;
mod ui;

fn main() -> Result<(), PlatformError> {
    pretty_env_logger::init();
    let err_msg = match check_xinput() {
        Ok(()) => None,
        Err(e) => Some(format!("{}", e)),
    };
    let main_window = WindowDesc::new(ui_builder(PcKeyboard::new()?))
        .title("Cleaboard")
        .window_size((450., 120.))
        .set_position((800., 200.));
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(State::new(true, err_msg))
}
