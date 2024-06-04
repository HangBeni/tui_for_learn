use std::fs;
use std::io;

use ratatui::widgets::ListState;

use super::types::Course;
use super::types::Error;
use super::types::User;

const COURSES_PATH: &str = "./src/db/courses.json";
const USERS_PATH: &str = "./src/db/users.json";

pub fn read_courses() -> Result<Vec<Course>, Error> {
    let db_courses = fs::read_to_string(COURSES_PATH)?;
    let parsed = serde_json::from_str(&db_courses)?;
    Ok(parsed)
}

pub fn get_course(course_id: usize) -> Course {
    read_courses()
        .unwrap()
        .iter()
        .find(|x| x.id == course_id )
        .unwrap()
        .clone()
}
pub fn read_users() -> Result<Vec<User>, Error> {
    let db_users = fs::read_to_string(USERS_PATH)?;
    let parsed = serde_json::from_str(&db_users)?;
    Ok(parsed)
}

pub fn add_course(new_course: Course) -> Result<bool, Error> {
    let mut courses = read_courses().unwrap();
    courses.push(new_course);
    fs::write(COURSES_PATH, &serde_json::to_string(&courses)?)?;
    Ok(true)
}

pub fn remove_course(course_list: &mut ListState) -> Result<(), Error> {
    if let Some(selected) = course_list.selected() {
        let mut courses = read_courses().unwrap();
        courses.remove(selected);
        fs::write(COURSES_PATH, &serde_json::to_string(&courses)?)?;
        course_list.select(Some(selected - 1))
    }
    Ok(())
}

pub fn check_code(code: &str) -> Result<&str, &str> {
    if code.chars().all(|ch| ch.is_alphanumeric()) {
        if code.len() == 6 {
            Ok("Neptun Code")
        } else {
            Err("The code is longer than 6 characters!ERROR")
        }
    } else {
        Err("Special character is not allowed in the Neptun code!ERROR")
    }
}

pub fn check_password(password: &str) -> Result<&str, &str> {
    if password.len() > 3 && password.chars().all(|ch| ch.is_alphanumeric()) {
        Ok("Password")
    } else if !password.chars().all(|ch| ch.is_alphanumeric()) && password.len() <= 3 {
        Err("Too short and there is a special character!ERROR")
    } else if password.len() <= 3 {
        Err("Too short the password, at least 4 character!ERROR")
    } else if !password.chars().all(|ch| ch.is_alphanumeric()) {
        Err("Special character!ERROR")
    } else {
        Err("Something wrong!ERROR")
    }
}

pub fn logger(code: &str, password: &str) -> Option<User> {
    if code.contains("ERROR") || password.contains("ERROR") {
        None
    } else {
        let users = read_users().unwrap();
        users
            .iter()
            .find(|user| {
                user.code.to_lowercase() == code.to_lowercase()
                    && user.password.to_lowercase() == password.to_lowercase()
            })
            .cloned()
    }
}

pub fn save_user(current_user: &User) -> Result<(), io::Error> {
    let mut users = read_users().unwrap();

     users.iter_mut().for_each(|user| {
        if user.code == current_user.code {
            user.user_schedule = current_user.user_schedule.clone();
        }
    });

    fs::write(USERS_PATH, &serde_json::to_string(&users)?)?;
    Ok(())
}
