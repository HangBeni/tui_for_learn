use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
};
use tui_for_learn::{
    components::{
        c_rect::centered_rect, exit::exit_popup, give_up::give_up_popup, nav::render_nav, status_bars::{course_help::courses_bar, movement::movement_bar, timetabel_help::timetable_bar},
    },
    pages::{
        courses::{render_courses, render_courses_with_taked},
        home::render_home,
        login::render_login,
        timetable::render_time_table,
    },
    util::types::{App, CurrentScreen, LoginState},
};

pub fn ui(f: &mut Frame, app: &App, courses: &mut ListState, login_state: &mut LoginState) {
    //Base layout
    let layout_area = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(80),
            Constraint::Length(3),
        ])
        .split(f.size());

    //Alap layout
    if login_state.user.is_some() {
        f.render_widget(
            render_nav(usize::from(app.current_screen) - 1),
            layout_area[0],
        );
    }

    //A képernyők
    match app.current_screen {
        CurrentScreen::Login => {
            render_login(f, app, login_state);
        }
        CurrentScreen::Home => {
            f.render_widget(render_home(login_state.user.clone()), layout_area[1]);
            f.render_widget(
                movement_bar(app.current_screen, &login_state.user.clone().unwrap()),
                layout_area[2]
            );
        }
        CurrentScreen::Courses => {
            f.render_widget(courses_bar(app.current_screen, &login_state.user.clone().unwrap()), layout_area[2]);
            match login_state.user.clone() {
            Some(mut user) => match user.user_schedule.len() {
                0 => render_courses(f, courses, layout_area),
                _ => {
                    render_courses_with_taked(
                        f,
                        courses,
                        layout_area,
                        &mut user,
                        app.current_course_list,
                    );
                }
            },
            _ => {}
        }},
        CurrentScreen::TimeTable => {
            f.render_widget(render_time_table(), layout_area[1]);
            f.render_widget(timetable_bar(app.current_screen, &login_state.user.clone().unwrap()), layout_area[2]);
        }
        CurrentScreen::Exiting => {
            f.render_widget(exit_popup(), centered_rect(40, 20, f.size()));
        }
        CurrentScreen::GiveUpCourse => {
            f.render_widget(give_up_popup(), centered_rect(40, 20, f.size()));
        }
    }
}
