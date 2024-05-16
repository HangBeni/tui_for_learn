use tui_for_learn::util::types::{CurrentScreen, LoginHighlight};

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_login_parameter: LoginHighlight,
    pub code_input: String,
    pub password_input: String,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Login,
            current_login_parameter: LoginHighlight::Neptun,
            code_input: String::new(),
            password_input: String::new(),
        }
    }
}
