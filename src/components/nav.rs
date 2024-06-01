use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Tabs},
};

use crate::util::types::CurrentScreen;

pub fn render_nav<'a>(screen: CurrentScreen) -> Tabs<'a> {
    let menu_titles = vec!["(1) Home", "(2) Courses", "(3) Timetable", "(q) Quit"];

    let menu = menu_titles.iter().map(|t| {
        let (highlighted, rest) = t.split_at(3);

        Line::from(vec![
            Span::styled(
                highlighted,
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled(rest, Style::default().fg(Color::White)),
        ])
    });

    let tabs = Tabs::new(menu)
        .select(screen as usize)
        .block(Block::bordered().title("Menu"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::LightBlue))
        .padding("  ", "  ");

    tabs
}
