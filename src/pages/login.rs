use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{components::c_rect::centered_rect, util::types::{App, LoginHighlight, LoginState}};

pub fn render_login(f: &mut Frame, app: &App, login_state: &mut LoginState) {
    let area = centered_rect(50, 50, f.size());
    let login_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(15)
        ])
        .split(area);


    let default_style = Style::default().bg(Color::LightMagenta).fg(Color::White);
    let error_style = Style::default().bg(Color::default()).fg(Color::White);
    let passed_style = Style::default().bg(Color::LightGreen).fg(Color::White);
    let active_style = Style::default().bg(Color::Reset).fg(Color::White);

    let mut code_block = Block::default()
        .title(login_state.neptun.clone())   
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let mut password_block = Block::default()
        .title(login_state.password.clone())
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    match &login_state.user {
        Some(user) => {
            code_block = code_block.style(passed_style).border_type(BorderType::Plain).title(user.name.clone());
            password_block = password_block.style(passed_style).border_type(BorderType::Plain);
        },
        None => {
            match app.current_login_parameter {
                LoginHighlight::Neptun =>{ 
                    match login_state.neptun.contains("ERROR") {
                    true => {
                        code_block = code_block
                            .style(error_style)
                            .border_style(Color::Red).border_type(BorderType::Double);
                    }
                    false => {
                        code_block = code_block
                            .style(active_style)
                            .border_style(Color::Green).border_type(BorderType::Double);
                    }
                }},
                LoginHighlight::Password => match login_state.password.contains("ERROR") {
                    true => {
                        password_block = password_block
                            .style(error_style)
                            .border_style(Color::Red).border_type(BorderType::Double);
                    }
                    false => {
                        password_block = password_block
                            .style(active_style)
                            .border_style(Color::Green).border_type(BorderType::Double);
                    }
                },
        
                LoginHighlight::None => {
                    code_block = code_block.style(default_style);
                    password_block = password_block.style(default_style);
                }
            }
        },
    }
   

    //Input field
    let code_text = Paragraph::new(app.code_input.clone()).block(code_block);
    f.render_widget(code_text, login_chunks[0]);
    //Input field
    let password_text = Paragraph::new(app.password_input.clone()).block(password_block);
    f.render_widget(password_text, login_chunks[1]);
}
