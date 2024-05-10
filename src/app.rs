use tui_for_learn::util::types::{Course, CurrentScreen};



pub struct App{
    pub current_screen: CurrentScreen,
}

impl App {

    pub fn new() -> App {
        App{ 
            current_screen: CurrentScreen::Home,
        }
    }
}

