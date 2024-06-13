use ratatui::{layout::Alignment, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph}};


pub fn give_up_popup<'a>() -> Paragraph<'a> {
    let popup_block = Block::default()
        .title("Are you sure?")
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .style(Style::default().fg(Color::White));

    let give_up_question = Paragraph::new("Do you want to give up this course? (Y/N)")
        .block(popup_block)
        .alignment(Alignment::Center);

    give_up_question
}




