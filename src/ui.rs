use std::rc::Rc;

use ratatui::{
    buffer::Cell,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        block::title, Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Row, Table,
        Tabs,
    },
    Frame,
};
use tui_for_learn::util::{db::read_courses, types::CurrentScreen};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &App, courses: &mut ListState) {
    //Base layout
    let layout_area = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
        ])
        .split(f.size());

    //Nav Widget
    let menu_titles = vec!["(1) Home", "(2) Courses", "(3) Timetable", "(q) Quit"];
    let active_menu: &CurrentScreen = &app.current_screen;

    let menu = menu_titles.iter().map(|t| {
        let (first, rest) = t.split_at(1);

        Line::from(vec![
            Span::styled(
                first,
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled(rest, Style::default().fg(Color::White)),
        ])
    });

    let tabs = Tabs::new(menu)
        .select((*active_menu).into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::LightBlue))
        .divider("|");

    //Status Widget
    let status_bar: Line = vec![
        "Location".into(),
        "|".into(),
        "Command status".into(),
        "|".into(),
        "Current Time".into(),
    ]
    .into();

    //Alap layout
    f.render_widget(tabs, layout_area[0]);
    f.render_widget(status_bar, layout_area[2]);

    //A képernyő kiválasztása
    match app.current_screen {
        CurrentScreen::Home => {
            f.render_widget(render_home(), layout_area[1]);
        }
        CurrentScreen::Courses => {
            let courses_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(layout_area[1]);

            let (list_of_courses, table) = render_courses(&courses);

            f.render_stateful_widget(list_of_courses, courses_layout[0], courses);
            f.render_widget(table, courses_layout[1]);

        }
        CurrentScreen::TimeTable => {
            f.render_widget(render_time_table(), layout_area[1]);
        }
        _ => {}
    }
}

//Home Widgets
fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new("Hang Benjámin")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        );
    home
}

//Courses Widgets
fn render_courses<'a>(courses_state: &ListState) -> (List<'a>, Table<'a>) {
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
                item.name.clone(),
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
            Constraint::Percentage(10),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(10)
        ],
    ).header(Row::new(vec![
        "ID",
        "Név",
        "Kód",
        "Típus"
    ]).style(Style::default().add_modifier(Modifier::BOLD)))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Details")
            .border_type(BorderType::Plain),
    );

    (list, details)
}

//TimeTable Widgets
fn render_time_table<'a>() -> Paragraph<'a> {
    let time_table = Paragraph::new("Hang Benjámin")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Timetable")
                .border_type(BorderType::Plain),
        );
    time_table
}
