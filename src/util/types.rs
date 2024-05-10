// az enum értékekhez lehet string értéket is társítani -- cargo add strum --features derive,strum_macros
//  #[strum(serialize = "érték")]

pub enum Lecture {
    Theory,
    Practical,
    Lab,
}
pub enum Role {
    Nebulo,
    Teacher,
}
pub enum CurrentScreen {
    Home,
    Courses,
    TimeTable,
    Exiting
}

impl From<CurrentScreen> for usize {
    fn from(value: CurrentScreen) -> usize {
        match value {
            CurrentScreen::Home => 1,
            CurrentScreen::Courses => 2,
            CurrentScreen::TimeTable => 3,
            CurrentScreen::Exiting => 4,
        }
    }
}

pub struct Course {
    id: usize,
    code: String, // egyedi azon. név+type
    lecture_type: Lecture,
    name: String,
    length: String,
}

pub struct User {
    id: usize,
    code: [char; 6],  //Neptun
    password: String, // jó lenne hashelve
    name: String,
    role: Role,
    user_schedule: Vec<String>, //az órák kódját kell eltárolni hogy ha a kurzus változik akkor változzon a hivatkozással
}
