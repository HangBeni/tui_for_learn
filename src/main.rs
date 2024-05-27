mod ui;
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
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
    handlers::{handle_deletion, handle_input, handle_navigation, handle_validation},
    types::{App, CurrentScreen, LoginHighlight, LoginState},
};

use crate::ui::ui;
use std::{error::Error, io};

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
                CurrentScreen::Login => match key.code {
                    KeyCode::Tab | KeyCode::Enter => handle_validation(app, &mut login_state),
                    KeyCode::Delete | KeyCode::Backspace => handle_deletion(app, &mut login_state),
                    KeyCode::Esc => app.current_login_parameter = LoginHighlight::None,
                    KeyCode::Char(char) => handle_input(char, app, &mut login_state),

                    _ => {}
                },
                //Home Event Handling
                CurrentScreen::Home => match key.code {
                    //Navigation
                    KeyCode::Char('2') | KeyCode::Char('3') | KeyCode::Char('q') => {
                        app.current_screen = handle_navigation(key.code)
                    }
                    //Navigation
                    _ => {}
                },
                //Courses Event Handling
                CurrentScreen::Courses => match key.code {
                    //Navigation
                    KeyCode::Char('1') | KeyCode::Char('3') | KeyCode::Char('q') => {
                        app.current_screen = handle_navigation(key.code)
                    }

                    //Navigation

                    //Selection
                    KeyCode::Up | KeyCode::Char('k') => {
                        if let Some(selected) = courses_list.selected() {
                            let course_list_length = read_courses().unwrap_or(Vec::new()).len();
                            if selected > 0 {
                                courses_list.select(Some(selected - 1));
                            } else {
                                courses_list.select(Some(course_list_length - 1));
                            }
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if let Some(selected) = courses_list.selected() {
                            let course_length = read_courses().unwrap_or(Vec::new()).len();
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
                    //Navigation
                    KeyCode::Char('1') | KeyCode::Char('2') | KeyCode::Char('q') => {
                        app.current_screen = handle_navigation(key.code)
                    }
                    //Navigation
                    _ => {}
                },
                //Exiting Event Handling
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    _ => {
                        if login_state.user.is_none() {
                            app.current_screen = CurrentScreen::Login;
                        } else {
                            app.current_screen = CurrentScreen::Home;
                        }
                    }
                },
            }
        }
    }
}
