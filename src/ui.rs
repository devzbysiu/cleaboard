use crate::keyboard::Keyboard;
use crate::state::State;
use druid::widget::{Button, Flex, Label};
use druid::{Env, Widget, WidgetExt};
use log::error;

pub(crate) fn ui_builder(keyboard: impl Keyboard) -> impl Widget<State> {
    let header = Label::new(|state: &State, _env: &Env| state.log_text())
        .with_text_size(18.)
        .padding(15.)
        .center();

    let btn_label = Label::new(|state: &State, _env: &Env| state.button_text()).with_text_size(18.);
    let button = Button::from_label(btn_label)
        .on_click(move |_ctx, state, _env| toggle_keyboard(state, keyboard))
        .disabled_if(|state: &State, _env: &Env| state.has_err())
        .padding(15.);

    Flex::column().with_child(header).with_child(button)
}

fn toggle_keyboard(state: &mut State, keyboard: impl Keyboard) {
    state.toggle();
    if state.enabled() {
        turn_on_keyboard(state, keyboard);
    } else {
        turn_off_keyboard(state, keyboard);
    }
}

fn turn_on_keyboard(state: &mut State, keyboard: impl Keyboard) {
    let res = keyboard.turn_on();
    if res.is_err() {
        error!("failed to turn on the keyboard: {:?}", res);
        state.err_msg("Error while turning on keyboard");
    }
}

fn turn_off_keyboard(state: &mut State, keyboard: impl Keyboard) {
    let res = keyboard.turn_off();
    if res.is_err() {
        error!("failed to turn off the keyboard: {:?}", res);
        state.err_msg("Error while turning off keyboard");
    }
}
