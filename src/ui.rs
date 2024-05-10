use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame,
};
use tui_for_learn::util::types::CurrentScreen;

use crate::app::App;
pub fn ui(f: &mut Frame, app: &App) {
    let layout_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
        ])
        .split(f.size());
    //Nav Widget
    let menu_titles = vec!["Home", "Courses", "Timetable"];
    let mut active_menu: CurrentScreen = CurrentScreen::Home;

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
        .select(active_menu.into())
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
    //Home Widgets

    let home = Paragraph::new("Hang Benjámin")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Plain),
        );

    //Courses Widgets

    //TimeTable Widgets

    //A képernyő kiválasztása
    f.render_widget(tabs, layout_areas[0]);

    f.render_widget(home, layout_areas[1]);

    f.render_widget(status_bar, layout_areas[2]);
}
