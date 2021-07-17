use druid::{AppLauncher, PlatformError, WindowDesc};

use state::State;
use ui::ui_builder;

fn main() -> Result<(), PlatformError> {
    keyboard::check_prerequisites()?;

    let main_window = WindowDesc::new(ui_builder())
        .title("Cleaboard")
        .window_size((450., 120.))
        .set_position((800., 200.));
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(State::default())
}

mod ui {
    use crate::keyboard;
    use crate::state::{button_text, header_text, State};
    use druid::widget::{Button, Flex, Label};
    use druid::{Env, Widget, WidgetExt};

    pub(crate) fn ui_builder() -> impl Widget<State> {
        let header = Label::new(|state: &State, _env: &Env| header_text(state.enabled))
            .with_text_size(18.)
            .padding(15.0)
            .center();

        let btn_label =
            Label::new(|state: &State, _env: &Env| button_text(state.enabled)).with_text_size(18.);
        let button = Button::from_label(btn_label)
            .on_click(|_ctx, state, _env| {
                state.enabled = !state.enabled;
                if state.enabled {
                    keyboard::turn_on().expect("failed to turn on the keyboard");
                } else {
                    keyboard::turn_off().expect("failed to turn off the keyboard");
                }
            })
            .padding(15.0);

        Flex::column().with_child(header).with_child(button)
    }
}

mod state {
    use druid::Data;

    const TURN_OFF_LABEL: &str = "Turn off the keyboard";
    const TURN_ON_LABEL: &str = "Turn on the keyboard";

    const TURNED_OFF_HEADER: &str = "Keyboard is turned off! You can start cleaning :)";
    const TURNED_ON_HEADER: &str = "Keyboard is turned on";

    #[derive(Clone, Data)]
    pub(crate) struct State {
        pub(crate) enabled: bool,
    }

    impl State {
        fn new(enabled: bool) -> Self {
            Self { enabled }
        }
    }

    pub(crate) fn button_text(enabled: bool) -> String {
        if enabled {
            TURN_OFF_LABEL.into()
        } else {
            TURN_ON_LABEL.into()
        }
    }

    pub(crate) fn header_text(enabled: bool) -> String {
        if enabled {
            TURNED_ON_HEADER.into()
        } else {
            TURNED_OFF_HEADER.into()
        }
    }

    impl Default for State {
        fn default() -> Self {
            Self::new(true)
        }
    }
}

mod keyboard {
    use anyhow::{bail, Result};
    use std::process::Command;

    pub(crate) fn check_prerequisites() -> Result<()> {
        match Command::new("xinput").arg("--version").status() {
            Ok(status) if status.success() => Ok(()),
            _ => {
                bail!("Looks like 'xinput' is not available. Please ensure it's working by running 'xinput --version'");
            }
        }
    }

    pub(crate) fn turn_off() -> Result<()> {
        Command::new("xinput").arg("float").arg("17").spawn()?;
        Ok(())
    }

    pub(crate) fn turn_on() -> Result<()> {
        Command::new("xinput")
            .arg("reattach")
            .arg("17")
            .arg("3")
            .spawn()?;
        Ok(())
    }
}
