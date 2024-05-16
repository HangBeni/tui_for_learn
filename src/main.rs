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
    db::{check_code, check_password, logger, read_courses},
    types::{CurrentScreen, LoginHighlight, LoginState},
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

//Az app folyamatát kezeli (lap váltás, input handle, login check stb.)
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    let mut courses_list = ListState::default();
    courses_list.select(Some(0));

    let mut login_state = LoginState::initialize();

    loop {
        terminal.draw(|f| ui(f, app, &mut courses_list, &mut login_state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                //Login Event Handling
                CurrentScreen::Login if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Tab | KeyCode::Enter => match app.current_login_parameter {
                        LoginHighlight::Neptun => {
                            let result = check_code(&app.code_input);
                            match result {
                                Ok(res) => {
                                    login_state.neptun = res.to_owned();
                                    if app.password_input.len() > 0 {
                                        login_state.user =
                                            logger(&app.code_input, &app.password_input);

                                        match login_state.user {
                                            Some(_) => app.current_screen = CurrentScreen::Home,
                                            None => {
                                                login_state.neptun = "Not Found!".to_owned();
                                                app.current_login_parameter =
                                                    LoginHighlight::Password;
                                            }
                                        }
                                    } else {
                                        app.current_login_parameter = LoginHighlight::Password;
                                    }
                                }
                                Err(err) => {
                                    login_state.neptun = err.to_owned();
                                    app.current_login_parameter = LoginHighlight::Password;
                                }
                            }
                        }

                        LoginHighlight::Password => {
                            let result = check_password(&app.password_input);
                            match result {
                                Ok(res) => {
                                    login_state.password = res.to_owned();
                                    if app.code_input.len() == 6 {
                                        login_state.user =
                                            logger(&app.code_input, &app.password_input);
                                        match login_state.user {
                                            Some(_) => app.current_screen = CurrentScreen::Home,
                                            None => {
                                                login_state.neptun = "Not Found!".to_owned();
                                                app.current_login_parameter =
                                                    LoginHighlight::Neptun;
                                            }
                                        }
                                    } else {
                                        app.current_login_parameter = LoginHighlight::Neptun;
                                    }
                                }
                                Err(err) => {
                                    login_state.password = err.to_owned();
                                    app.current_login_parameter = LoginHighlight::Neptun;
                                }
                            }
                        }

                        LoginHighlight::None => {
                            app.current_login_parameter = LoginHighlight::Neptun
                        }
                    },
                    KeyCode::Backspace => match app.current_login_parameter {
                        LoginHighlight::Neptun => {
                            if login_state.password != "Neptun Code".to_owned() {
                                login_state.password = "Neptun Code".to_owned()
                            };
                            app.code_input.pop();

                            let result = check_code(&app.code_input);

                            match result {
                                Ok(res) => {
                                    login_state.neptun = res.to_owned();
                                }
                                Err(err) => {
                                    login_state.neptun = err.to_owned();
                                }
                            }
                        }
                        LoginHighlight::Password => {
                            if login_state.password != "Password".to_owned() {
                                login_state.password = "Password".to_owned()
                            };
                            app.password_input.pop();

                            let result = check_password(&app.password_input);

                            match result {
                                Ok(res) => {
                                    login_state.password = res.to_owned();
                                }
                                Err(err) => {
                                    login_state.password = err.to_owned();
                                }
                            }
                        }
                        _ => {}
                    },

                    KeyCode::Char(char) => match app.current_login_parameter {
                        LoginHighlight::Neptun => {

                            if login_state.neptun != "Neptun Code".to_owned() {
                                login_state.neptun = "Neptun Code".to_owned()
                            };

                            app.code_input.push(char);

                            if app.code_input.len().ge(&6) || !char.is_alphanumeric() {
                                let result = check_code(&app.code_input);

                                match result {
                                    Ok(res) => {
                                        login_state.neptun = res.to_owned();
                                    }
                                    Err(err) => {
                                        login_state.neptun = err.to_owned();
                                    }
                                }
                            }
                        }

                        LoginHighlight::Password => {
                            if login_state.password != "Password".to_owned() {
                                login_state.password = "Password".to_owned()
                            };

                            app.password_input.push(char);

                        
                                let result = check_password(&app.password_input);

                                match result {
                                    Ok(res) => {
                                        login_state.password = res.to_owned();
                                    }
                                    Err(err) => {
                                        login_state.password = err.to_owned();
                                    }
                            }
                        }
                        LoginHighlight::None => {
                            if char == 'q' {
                                app.current_screen = CurrentScreen::Exiting
                            };
                        }
                    },
                    KeyCode::Esc => app.current_login_parameter = LoginHighlight::None,
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
