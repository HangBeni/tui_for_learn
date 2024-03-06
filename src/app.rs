pub enum CurrentScreen {
    Home,
    Course,
    TimeTable
}

pub struct Course {
    pub  code:String,
    pub name: String,
    pub scheduled:String 
}  
pub struct App{
    pub current_screen: CurrentScreen,
    pub courses: Vec<Course>
}

impl App {

    pub fn new() -> App {
        App{ 
            current_screen: CurrentScreen::Home,
            courses: Vec::new(),
        }
    }
}

