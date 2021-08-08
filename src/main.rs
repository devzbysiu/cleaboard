use druid::{AppLauncher, PlatformError, WindowDesc};

use keyboard::PcKeyboard;
use state::State;
use ui::ui_builder;

mod keyboard;
mod state;
mod ui;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder(PcKeyboard::default()))
        .title("Cleaboard")
        .window_size((450., 120.))
        .set_position((800., 200.));
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(State::default())
}
