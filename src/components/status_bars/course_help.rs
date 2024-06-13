use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::util::types::{CurrentScreen, User};

pub fn courses_bar<'a>(current_screen: CurrentScreen, user: &User) -> Paragraph<'a> {
    let status_bar: Line = Line::from(vec![
        format!("User Code: {}", user.code).into(),
        format!(" | Navigation: <j/k> <arrow keys> | <enter/a> sign up | <r> remove | <tab> switch | ").into(),
        format!(
            "Current Time: {}",
            chrono::Local::now().format("%H:%M:%S %Y-%m-%d")
        )
        .into(),
        format!(" | Current Screen: {:?}", current_screen).into()
    ]);

    let block = Block::bordered()
        .style(Style::default().fg(Color::White))
        .title("Status Bar");
    let status_bar = Paragraph::new(status_bar).block(block);
    status_bar
}
