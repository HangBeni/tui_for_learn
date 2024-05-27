use std::rc::Rc;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, BorderType, Borders, List, ListItem, ListState, Row, Table}, Frame};

use crate::util::db::read_courses;


pub fn render_courses(f: &mut Frame, courses:&mut ListState, layout_area: Rc<[Rect]> ) {
    let courses_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
    .split(layout_area[1]);

let (list_of_courses, table) = courses_widget(&courses);

f.render_stateful_widget(list_of_courses, courses_layout[0],  courses);
f.render_widget(table, courses_layout[1]);
}
//Courses Widgets
fn courses_widget<'a>(courses_state: &ListState) -> (List<'a>, Table<'a>) {
    let courses = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Courses")
        .border_type(BorderType::Plain);

    let course_list = read_courses().expect("can fetch courses");

    let course_tags: Vec<ListItem> = course_list
        .iter()
        .map(|item| {
            ListItem::new(Line::from(vec![Span::styled(
                item.code.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_course = course_list
        .get(
            courses_state
                .selected()
                .expect("There is always a selected course"),
        )
        .expect("Exists")
        .clone();

    let list = List::new(course_tags).block(courses).highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );

    let details = Table::new(
        [Row::new(vec![
            selected_course.id.to_string(),
            selected_course.name,
            selected_course.code,
            selected_course.lecture_type.to_string(),
        ])],
        &[
            Constraint::Percentage(8),
            Constraint::Percentage(45),
            Constraint::Percentage(30),
            Constraint::Percentage(17),
        ],
    )
    .header(
        Row::new(vec!["ID", "Name", "Code", "Type"])
            .style(Style::default().add_modifier(Modifier::BOLD)),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Details")
            .border_type(BorderType::Plain),
    );

    (list, details)
}
