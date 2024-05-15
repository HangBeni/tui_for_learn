use tui_for_learn::util::types::{CurrentScreen, LoginHighlights, LoginValidation};

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_login_parameter: LoginHighlights,
    pub code_input: String,
    pub password_input: String,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Login,
            current_login_parameter: LoginHighlights::Neptun {
                valid: LoginValidation::Pending,
            },
            code_input: String::new(),
            password_input: String::new(),
        }
    }
}
