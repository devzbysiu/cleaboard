use druid::Data;

const TURN_OFF_LABEL: &str = "Turn off the keyboard";
const TURN_ON_LABEL: &str = "Turn on the keyboard";

const TURNED_OFF_HEADER: &str = "Keyboard is turned off! You can start cleaning :)";
const TURNED_ON_HEADER: &str = "Keyboard is turned on";

#[derive(Clone, Data)]
pub(crate) struct State {
    enabled: bool,
    err_msg: Option<String>,
}

impl State {
    pub(crate) fn new(enabled: bool, err_msg: Option<String>) -> Self {
        Self { enabled, err_msg }
    }

    pub(crate) fn button_text(&self) -> String {
        if self.enabled {
            TURN_OFF_LABEL.into()
        } else {
            TURN_ON_LABEL.into()
        }
    }

    pub(crate) fn log_text(&self) -> String {
        match &self.err_msg {
            Some(msg) => msg.into(),
            None => {
                if self.enabled {
                    TURNED_ON_HEADER.into()
                } else {
                    TURNED_OFF_HEADER.into()
                }
            }
        }
    }

    pub(crate) fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn has_err(&self) -> bool {
        self.err_msg.is_some()
    }

    pub(crate) fn err_msg<S: Into<String>>(&mut self, msg: S) {
        self.err_msg = Some(msg.into());
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(true, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_button_text() {
        // given
        let state = State::default();

        // when
        let text = state.button_text();

        // then
        assert_eq!(text, "Turn off the keyboard");
    }

    #[test]
    fn test_button_text_when_keyboard_disabled() {
        // given
        let mut state = State::default();
        state.toggle();

        // when
        let text = state.button_text();

        // then
        assert_eq!(text, "Turn on the keyboard");
    }

    #[test]
    fn test_log_text() {
        // given
        let state = State::default();

        // when
        let msg = state.log_text();

        // then
        assert_eq!(msg, "Keyboard is turned on");
    }

    #[test]
    fn test_log_text_when_keyboard_disabled() {
        // given
        let mut state = State::default();
        state.toggle();

        // when
        let msg = state.log_text();

        // then
        assert_eq!(msg, "Keyboard is turned off! You can start cleaning :)");
    }

    #[test]
    fn test_default_state() {
        // given
        let state = State::default();

        // when
        let enabled = state.enabled();

        // then
        assert!(enabled);
    }

    #[test]
    fn test_toggle() {
        // given
        let mut state = State::default();
        assert!(state.enabled());

        // when
        state.toggle();
        assert!(!state.enabled());
        state.toggle();

        // then
        assert!(state.enabled());
    }
}
