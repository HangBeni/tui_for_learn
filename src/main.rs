mod ui;
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, 
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::ListState,
    Terminal,
};
use tui_for_learn::util::{
    db::{read_courses, save_user},
    handlers::{handle_deletion, handle_input, handle_navigation, handle_validation},
    helper_functions::switch_list,
    types::{App, CourseList, CurrentScreen, LoginHighlight, LoginState},
};

use crate::ui::ui;
use std::{error::Error, io::{self}};

fn main() -> io::Result<()> {
    //terminal setup
    enable_raw_mode()?;
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    // execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    ratatui::restore();

    res
}

//Az app folyamatát kezeli (lap váltás, input handle, login check stb.)
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut courses_list = ListState::default();
    courses_list.select_first();

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
                    KeyCode::Right => {
                        app.current_course_list = CourseList::TakedCourses;
                    }
                    KeyCode::Left => {
                        app.current_course_list = CourseList::AllCourses;
                    }
                    KeyCode::Tab => {
                        app.current_course_list = match app.current_course_list {
                            CourseList::TakedCourses => {
                                match switch_list(
                                    courses_list.selected(),
                                    read_courses().unwrap_or(Vec::new()).len(),
                                ) {
                                    Some(selected) => {
                                        courses_list.select(Some(selected));
                                        CourseList::AllCourses
                                    }
                                    None => CourseList::AllCourses,
                                }
                            }
                            CourseList::AllCourses => {
                                match switch_list(
                                    courses_list.selected(),
                                    login_state.user.clone().unwrap().user_schedule.len(),
                                ) {
                                    Some(selected) => {
                                        courses_list.select(Some(selected));
                                        CourseList::TakedCourses
                                    }
                                    None => CourseList::AllCourses,
                                }
                            }
                        }
                    }
                    //Add a course to the user schedule
                    KeyCode::Char('a') | KeyCode::Enter => login_state
                        .user
                        .as_mut()
                        .unwrap()
                        .add_course(courses_list.selected().unwrap()),
                    //Remove a course from the user schedule
                    KeyCode::Char('r') => {
                        app.current_screen = CurrentScreen::GiveUpCourse;
                    }
                    //Navigate between courses
                    KeyCode::Up | KeyCode::Char('k') => {
                        if let Some(selected) = courses_list.selected() {
                            match app.current_course_list {
                                CourseList::TakedCourses => {
                                    if selected > 0 {
                                        courses_list.select_previous();
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
                                        courses_list.select_previous();
                                    } else {
                                        courses_list.select(Some(course_list_length - 1));
                                    }
                                }
                            }
                        }
                    }
                    //Navigate between courses
                    KeyCode::Down | KeyCode::Char('j') => {
                        if let Some(selected) = courses_list.selected() {
                            match app.current_course_list {
                                CourseList::TakedCourses => {
                                    if selected + 1
                                        >= login_state.user.clone().unwrap().user_schedule.len()
                                    {
                                        courses_list.select_first();
                                    } else {
                                        courses_list.select_next();
                                    }
                                }
                                CourseList::AllCourses => {
                                    let course_length = read_courses().unwrap_or(Vec::new()).len();
                                    if selected + 1 >= course_length {
                                        courses_list.select_first();
                                    } else {
                                        courses_list.select_next();
                                    }
                                }
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
                        return Ok(());
                    }
                    _ => {
                        if login_state.user.is_none() {
                            app.current_screen = CurrentScreen::Login;
                        } else {
                            app.current_screen = CurrentScreen::Home;
                        }
                    }
                },
                CurrentScreen::GiveUpCourse => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('r') => {
                        match switch_list(
                            courses_list.selected(),
                            login_state.user.clone().unwrap().user_schedule.len(),
                        ) {
                            Some(selected) => {
                                login_state
                                    .user
                                    .as_mut()
                                    .unwrap()
                                    .user_schedule
                                    .remove(selected);
                                if login_state.user.clone().unwrap().user_schedule.len() == 0 {
                                    app.current_course_list = CourseList::AllCourses;
                                    courses_list.select(Some(0));
                                }
                                app.current_screen = CurrentScreen::Courses;
                            }
                            None => {
                                app.current_course_list = CourseList::AllCourses;
                                app.current_screen = CurrentScreen::Courses;
                            }
                        }
                    }
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Courses;
                    }
                    _ => {}
                },
            }
        }
    }
}
