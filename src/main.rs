mod app;
mod ui;
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::ListState,
    Terminal,
};
use tui_for_learn::util::{
    db::read_courses,
    types::{CurrentScreen, LoginHighlights, LoginValidation},
};

use crate::{app::App, ui::ui};
use std::{
    error::Error,
    io::{self},
};

fn main() -> Result<(), Box<dyn Error>> {
    //terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

//Az app folyamatát kezeli (lap váltás stb.)
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    let mut courses_list = ListState::default();
    courses_list.select(Some(0));

    loop {
        terminal.draw(|f| ui(f, app, &mut courses_list))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                //Login Event Handling
                CurrentScreen::Login if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Tab | KeyCode::Enter => match &app.current_login_parameter {
                        LoginHighlights::Neptun { valid:_ } => {
                            
                            app.current_login_parameter = LoginHighlights::Password{valid :LoginValidation::Pending } ;
                        }
                        LoginHighlights::Password { valid: _ } => {
                            
                            app.current_login_parameter = LoginHighlights::Neptun { valid: LoginValidation::Pending } ;
                        }
                        LoginHighlights::None => app.current_login_parameter = LoginHighlights::Neptun { valid: LoginValidation::Pending },
                    },
                    KeyCode::Backspace => match app.current_login_parameter {
                        LoginHighlights::Neptun { valid: _ } => {
                            app.code_input.pop();
                        }
                        LoginHighlights::Password { valid: _ } => {
                            app.password_input.pop();
                        }
                        LoginHighlights::None => {
                            if key.code == KeyCode::Char('q') {
                                app.current_screen = CurrentScreen::Exiting
                            }
                        }
                    },
                    KeyCode::Char(char) => match app.current_login_parameter {
                        LoginHighlights::Neptun { valid: _ } => {
                            app.code_input.push(char);
                        }
                        LoginHighlights::Password { valid: _ } => {
                            app.password_input.push(char);
                        }
                        LoginHighlights::None => {
                            if key.code == KeyCode::Char('q') {
                                app.current_screen = CurrentScreen::Exiting
                            }
                        }
                    },
                    KeyCode::Esc => app.current_login_parameter = LoginHighlights::None,
                    _ => {}
                },
                //Home Event Handling
                CurrentScreen::Home => match key.code {
                    KeyCode::Char('2') => app.current_screen = CurrentScreen::Courses,
                    KeyCode::Char('3') => app.current_screen = CurrentScreen::TimeTable,
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                    _ => {}
                },
                //Courses Event Handling
                CurrentScreen::Courses => match key.code {
                    KeyCode::Char('1') => app.current_screen = CurrentScreen::Home,
                    KeyCode::Char('3') => app.current_screen = CurrentScreen::TimeTable,
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                    KeyCode::Up | KeyCode::Char('j') => {
                        if let Some(selected) = courses_list.selected() {
                            let course_list_length = read_courses().expect("can fetch").len();
                            if selected > 0 {
                                courses_list.select(Some(selected - 1));
                            } else {
                                courses_list.select(Some(course_list_length - 1));
                            }
                        }
                    }
                    KeyCode::Down | KeyCode::Char('k') => {
                        if let Some(selected) = courses_list.selected() {
                            let course_length = read_courses().expect("can fetch").len();
                            if selected >= course_length - 1 {
                                courses_list.select(Some(0));
                            } else {
                                courses_list.select(Some(selected + 1));
                            }
                        }
                    }
                    _ => {}
                },
                //Timetable Event Handling
                CurrentScreen::TimeTable => match key.code {
                    KeyCode::Char('1') => app.current_screen = CurrentScreen::Home,
                    KeyCode::Char('2') => app.current_screen = CurrentScreen::Courses,
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                    _ => {}
                },
                //Exiting Event Handling
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    _ => app.current_screen = CurrentScreen::Home,
                },
                _ => {}
            }
        }
    }
}
