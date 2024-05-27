use ratatui::{style::{Color, Style}, widgets::{Block, Borders, Paragraph, Wrap}};


pub fn exit_popup() -> Paragraph<'static> {
    let popup_block = Block::default()
        .title("Y/N")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    // the `trim: false` will stop the text from being cut off when over the edge of the block
    Paragraph::new("Ki akarsz lépni?")
        .block(popup_block)
        .wrap(Wrap { trim: false })
}
