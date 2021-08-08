use crate::keyboard::Keyboard;
use crate::state::State;
use druid::widget::{Button, Flex, Label};
use druid::{Env, Widget, WidgetExt};

pub(crate) fn ui_builder<K>(keyboard: K) -> impl Widget<State>
where
    K: Keyboard + Copy + 'static,
{
    let header = Label::new(|state: &State, _env: &Env| state.log_text())
        .with_text_size(18.)
        .padding(15.)
        .center();

    let btn_label = Label::new(|state: &State, _env: &Env| state.button_text()).with_text_size(18.);
    let button = Button::from_label(btn_label)
        .on_click(move |_ctx, state, _env| {
            toggle_keyboard(state, keyboard);
        })
        .disabled_if(|state: &State, _env: &Env| state.has_err())
        .padding(15.);

    Flex::column().with_child(header).with_child(button)
}

fn toggle_keyboard<K>(state: &mut State, keyboard: K)
where
    K: Keyboard + Copy + 'static,
{
    state.toggle();
    if state.enabled() {
        println!("enabling keyboard");
        keyboard.turn_on().expect("failed to turn on the keyboard");
    } else {
        println!("disabling keyboard");
        keyboard
            .turn_off()
            .expect("failed to turn off the keyboard");
    }
}
