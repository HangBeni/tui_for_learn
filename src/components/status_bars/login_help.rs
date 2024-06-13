use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::{Block, Paragraph},
};

pub fn login_bar<'a>() -> Paragraph<'a> {
    let status_bar: Line = Line::from(vec![
        format!("<Tab/Enter> switch parameter and login").into()
    ]);

    let block = Block::bordered()
        .style(Style::default().fg(Color::White))
        .title("Tooltip");
    let status_bar = Paragraph::new(status_bar).block(block);
    status_bar
}
