use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Row, Table},
    Frame,
};

use crate::util::{
    db::{get_course, read_courses},
    types::{Course, CourseList, User},
};

pub fn render_courses(f: &mut Frame, courses: &mut ListState, layout_area: Rc<[Rect]>) {
    let courses_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Fill(1)].as_ref())
        .split(layout_area[1]);

    let (list_of_courses, table, _) = courses_widget(courses, None);

    f.render_stateful_widget(list_of_courses, courses_layout[0], courses);
    f.render_widget(table, courses_layout[1]);
}

pub fn render_courses_with_taked(
    f: &mut Frame,
    courses: &mut ListState,
    layout_area: Rc<[Rect]>,
    user: &mut User,
    selected_list: CourseList,
) {
    let courses_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(30),
                Constraint::Fill(1),
                Constraint::Length(30),
            ]
            .as_ref(),
        )
        .split(layout_area[1]);

    let (list_of_courses, table, taked_courses_list) =
        courses_widget(courses, Some(&user.user_schedule));
    match selected_list {
        CourseList::TakedCourses => {
            f.render_widget(list_of_courses, courses_layout[0]);
            f.render_widget(table, courses_layout[1]);
            f.render_stateful_widget(taked_courses_list, courses_layout[2], courses);
        }
        CourseList::AllCourses => {
            f.render_stateful_widget(list_of_courses, courses_layout[0], courses);
            f.render_widget(table, courses_layout[1]);
            f.render_widget(taked_courses_list, courses_layout[2]);
        }
    }
}

fn courses_widget<'a>(
    courses_state: &mut ListState,
    taked_courses: Option<&Vec<usize>>,
) -> (List<'a>, Table<'a>, List<'a>) {
    let course_list = read_courses().expect("can fetch courses");

    let selected_course = course_list
        .get(
            courses_state
                .selected()
                .expect("There is always a selected course"),
        )
        .expect("Exists")
        .clone();

    (
        available_courses_list(&course_list),
        details_table(&selected_course),
        taked_courses_list(taked_courses),
    )
}

fn details_table<'a>(selected_course: &Course) -> Table<'a> {
    let details = Table::new(
        [Row::new(vec![
            selected_course.id.to_string(),
            selected_course.code.clone(),
            selected_course.takes_on.to_string(),
        ])],
        &[
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ],
    )
    .header(
        Row::new(vec!["ID", "Code", "Take on"])
            .style(Style::default().add_modifier(Modifier::BOLD)),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Details")
            .border_type(BorderType::Plain),
    );

    details
}

//Display all the available courses
fn available_courses_list<'a>(course_list: &Vec<Course>) -> List<'a> {
    let courses = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Courses")
        .border_type(BorderType::Plain);

    let course_tags: Vec<ListItem> = course_list
        .iter()
        .map(|item| {
            ListItem::new(Line::from(vec![
                Span::styled(item.code.to_string(), Style::default()),
                Span::styled(" ", Style::default()),
                Span::styled(item.takes_on.to_string(), Style::default()),
            ]))
        })
        .collect();

    let list = List::new(course_tags).block(courses).highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );

    list
}
//Display taked courses
fn taked_courses_list<'a>(taked_courses: Option<&Vec<usize>>) -> List<'a> {
    let course_list: Vec<Course> = taked_courses
        .unwrap_or(&Vec::new())
        .iter()
        .map(|id| get_course(*id - 1))
        .collect();

    let courses = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Taken Courses")
        .border_type(BorderType::Plain);

    let course_tags: Vec<ListItem> = course_list
        .iter()
        .map(|item| {
            ListItem::new(Line::from(vec![
                Span::styled(item.code.to_string(), Style::default()),
                Span::styled(" ", Style::default()),
                Span::styled(item.takes_on.to_string(), Style::default()),
            ]))
        })
        .collect();

    let list = List::new(course_tags).block(courses).highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );

    list
}
