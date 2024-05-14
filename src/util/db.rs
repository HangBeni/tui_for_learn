use std::fs;

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

pub fn read_users() -> Result<Vec<User>, Error> {
    let db_users = fs::read_to_string(USERS_PATH)?;
    let parsed = serde_json::from_str(&db_users)?;
    Ok(parsed)
}

pub fn add_course(new_course: Course) -> Result<bool, Error> {
    let mut courses = read_courses().unwrap();
    courses.push(new_course);
    fs::write(COURSES_PATH, &serde_json::to_vec(&courses)?)?;
    Ok(true)
}

pub fn remove_course(course_list: &mut ListState) -> Result<(), Error> {
    if let Some(selected) = course_list.selected() {
        let mut courses = read_courses().unwrap();
        courses.remove(selected);
        fs::write(COURSES_PATH, &serde_json::to_vec(&courses)?)?;
        course_list.select(Some(selected - 1))
    }
    Ok(())
}
