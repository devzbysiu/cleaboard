use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, Env, PlatformError, Widget, WidgetExt, WindowDesc};

const TURN_OFF_LABEL: &str = "Turn off the keyboard";
const TURN_ON_LABEL: &str = "Turn on the keyboard";

const TURNED_OFF_HEADER: &str = "Keyboard is turned off! You can start cleaning :)";
const TURNED_ON_HEADER: &str = "Keyboard is turned on";

#[derive(Clone, Data)]
struct State {
    enabled: bool,
}

impl State {
    fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

fn get_button_text(enabled: bool) -> String {
    if enabled {
        TURN_OFF_LABEL.into()
    } else {
        TURN_ON_LABEL.into()
    }
}

fn get_header_text(enabled: bool) -> String {
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

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder())
        .title("Cleaboard")
        .window_size((450., 120.))
        .set_position((800., 200.));
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(State::default())
}

fn ui_builder() -> impl Widget<State> {
    let header = Label::new(|state: &State, _env: &Env| get_header_text(state.enabled))
        .with_text_size(18.)
        .padding(15.0)
        .center();

    let btn_label =
        Label::new(|state: &State, _env: &Env| get_button_text(state.enabled)).with_text_size(18.);
    let button = Button::from_label(btn_label)
        .on_click(|_ctx, state, _env| state.enabled = !state.enabled)
        .padding(15.0);

    Flex::column().with_child(header).with_child(button)
}
