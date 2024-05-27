use ratatui::{layout::Alignment, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph}};


pub fn render_time_table<'a>() -> Paragraph<'a> {
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