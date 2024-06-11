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
    db::{read_courses, save_user},
    handlers::{handle_deletion, handle_input, handle_navigation, handle_validation},
    types::{App, CourseList, CurrentScreen, LoginHighlight, LoginState},
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
                        app.current_screen = handle_navigation(key.code);
                        app.current_course_list = CourseList::None;
                    }
                    _ => {}
                },
                //Courses Event Handling
                CurrentScreen::Courses => match key.code {
                    //Navigation
                    KeyCode::Char('1') | KeyCode::Char('3') | KeyCode::Char('q') => {
                        app.current_screen = handle_navigation(key.code)
                    }
                    //Switch course list
                    KeyCode::Tab => {
                        app.current_course_list = match app.current_course_list {
                            CourseList::TakedCourses => {
                                if courses_list.selected().unwrap() <= read_courses().unwrap().len()
                                {
                                    courses_list.select(courses_list.selected());
                                } else {
                                    courses_list.select(Some(0));
                                }
                                CourseList::AllCourses
                            }
                            CourseList::AllCourses => {
                                if login_state.user.clone().unwrap().user_schedule.len()
                                    >= courses_list.selected().unwrap()
                                {
                                    courses_list.select(courses_list.selected());
                                } else {
                                    courses_list.select(Some(0));
                                }
                                CourseList::TakedCourses
                            }
                            CourseList::None => {
                                courses_list.select(Some(0));
                                CourseList::AllCourses
                            }
                        }
                    }
                    //Add a course to the user schedule
                    KeyCode::Char('a') | KeyCode::Enter => login_state
                        .user
                        .as_mut()
                        .unwrap()
                        .add_course(courses_list.selected().unwrap() + 1),
                    //Navigate between courses
                    KeyCode::Up | KeyCode::Char('k') => {
                        if let Some(selected) = courses_list.selected() {
                            match app.current_course_list {
                                CourseList::TakedCourses => {
                                    if selected > 0 {
                                        courses_list.select(Some(selected - 1));
                                    } else {
                                        courses_list.select(Some(
                                            login_state.user.clone().unwrap().user_schedule.len()
                                                - 1,
                                        ));
                                    }
                                }
                                CourseList::AllCourses => {
                                    let course_list_length =
                                        read_courses().unwrap_or(Vec::new()).len();
                                    if selected > 0 {
                                        courses_list.select(Some(selected - 1));
                                    } else {
                                        courses_list.select(Some(course_list_length - 1));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    //Navigate between courses
                    KeyCode::Down | KeyCode::Char('j') => {
                        if let Some(selected) = courses_list.selected() {
                            match app.current_course_list {
                                CourseList::TakedCourses => {
                                    if selected
                                        >= login_state.user.clone().unwrap().user_schedule.len() - 1
                                    {
                                        courses_list.select(Some(0));
                                    } else {
                                        courses_list.select(Some(selected + 1));
                                    }
                                }
                                CourseList::AllCourses => {
                                    let course_length = read_courses().unwrap_or(Vec::new()).len();
                                    if selected >= course_length - 1 {
                                        courses_list.select(Some(0));
                                    } else {
                                        courses_list.select(Some(selected + 1));
                                    }
                                }
                                _ => {}
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
                        if login_state.user.clone().is_some() {
                            save_user(&login_state.user.clone().unwrap())?
                        }
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
