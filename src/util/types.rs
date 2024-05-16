use strum::Display;

// az enum értékekhez lehet string értéket is társítani -- cargo add strum --features derive,strum_macros
//  #[strum(serialize = "érték")]

use std::io;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Display)]
pub enum Lecture {
    #[strum(serialize = "Eloadás")]
    Theory,
    #[strum(serialize = "Gyakorlat")]
    Practical,
    #[strum(serialize = "Labor")]
    Lab,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum Role {
    Nebulo,
    Teacher,
}
#[derive(Copy, Clone, Debug)]
pub enum CurrentScreen {
    Home,
    Courses,
    TimeTable,
    Exiting,
    Login,
}

impl From<CurrentScreen> for usize {
    fn from(value: CurrentScreen) -> usize {
        match value {
            CurrentScreen::Login => 0,
            CurrentScreen::Home => 1,
            CurrentScreen::Courses => 2,
            CurrentScreen::TimeTable => 3,
            CurrentScreen::Exiting => 4,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LoginHighlight {
    Neptun,
    Password,
    None,
}

pub struct LoginState{
   pub neptun:String, //Display if the check is not passed through
   pub password:String, //Display if the check is not passed through
   pub user: Option<User> // The logged user
}
impl LoginState {
    pub fn initialize() ->  LoginState {
        LoginState{
            neptun:"Neptun Code".to_owned(),
            password: "Password".to_owned(),
            user: None
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
pub struct Course {
    pub id: usize,
    pub code: String, // egyedi azon. név+type
    pub lecture_type: Lecture,
    pub name: String,
    pub length: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: usize,
    pub code: String,  //Neptun
    pub password: String, // jó lenne hashelve
    pub name: String,
    pub role: Role,
    pub uni: String,
    pub faculty: String,
    pub major: String,
    pub user_schedule: Vec<String>, //az órák kódját kell eltárolni hogy ha a kurzus változik akkor változzon a hivatkozással
}
