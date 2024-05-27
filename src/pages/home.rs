use ratatui::{layout::Alignment, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph}};

use crate::util::types::User;


//Home Widgets
pub fn render_home<'a>(user: Option<User>) -> Paragraph<'a> {
    let home = Paragraph::new(user.unwrap().name)
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