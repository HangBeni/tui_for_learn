use strum::Display;

// az enum értékekhez lehet string értéket is társítani -- cargo add strum --features derive,strum_macros
//  #[strum(serialize = "érték")]

use core::fmt;
use std::io;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::db::get_course;

#[derive(Serialize, Deserialize, Clone, Display)]
pub enum Lecture {
    #[strum(serialize = "Theory")]
    Theory,
    #[strum(serialize = "Practical")]
    Practical,
    #[strum(serialize = "Lab")]
    Lab,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum Role {
    Nebulo,
    Teacher,
}
#[derive(Copy, Clone, Debug)]
pub enum CurrentScreen {
    Login,
    Home,
    Courses,
    TimeTable,
    Exiting,
    GiveUpCourse,
}
impl From<CurrentScreen> for usize {
    fn from(value: CurrentScreen) -> usize {
        match value {
            CurrentScreen::Login => 0,
            CurrentScreen::Home => 1,
            CurrentScreen::Courses => 2,
            CurrentScreen::TimeTable => 3,
            CurrentScreen::Exiting => 4,
            CurrentScreen::GiveUpCourse => 5,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LoginHighlight {
    Neptun,
    Password,
    None,
}
#[derive(Clone)]
pub struct LoginState {
    pub neptun: String,     //Display if the check is not passed through
    pub password: String,   //Display if the check is not passed through
    pub user: Option<User>, // The logged user
}
impl LoginState {
    pub fn initialize() -> LoginState {
        LoginState {
            neptun: "Neptun Code".to_owned(),
            password: "Password".to_owned(),
            user: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}
#[derive(Serialize, Deserialize, Clone)]
pub struct CourseTime {
    // Órák meghirdetett idopontjának struktúrája
    pub day: String,
    pub time: String,
}
impl fmt::Display for CourseTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.day, self.time)
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Course {
    pub id: usize,
    pub code: String, // név+kód+type
    pub lecture_type: Lecture,
    pub name: String,
    pub takes_on: CourseTime,
    pub length: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: usize,
    pub code: String,     //Neptun
    pub password: String, // jó lenne hashelve
    pub name: String,
    pub role: Role,
    pub uni: String,
    pub faculty: String,
    pub major: String,
    pub user_schedule: Vec<usize>, //az órák id kell eltárolni hogy ha a kurzus változik akkor változzon a hivatkozással
}
impl User {
    pub fn add_course(&mut self, course_index: usize) {
        let course_id = get_course(course_index).id;

        if !self.user_schedule.contains(&course_id) {
            self.user_schedule.push(course_id);
        }
    }

    pub fn get_course(&mut self, course_index: usize) -> Course {
        get_course(course_index)
    }
}
#[derive(Clone, Copy)]
pub enum CourseList {
    TakedCourses,
    AllCourses,
}
pub struct App {
    pub current_screen: CurrentScreen,
    pub current_login_parameter: LoginHighlight,
    pub current_course_list: CourseList,
    pub code_input: String,
    pub password_input: String,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Login,
            current_login_parameter: LoginHighlight::Neptun,
            current_course_list: CourseList::AllCourses,
            code_input: String::new(),
            password_input: String::new(),
        }
    }
}
