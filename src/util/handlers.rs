use crossterm::event::KeyCode;

use super::{
    db::{check_code, check_password, logger},
    types::{App, CurrentScreen, LoginHighlight, LoginState},
};

pub fn handle_validation(app: &mut App, login_state: &mut LoginState) {

    match app.current_login_parameter {

        LoginHighlight::Neptun => {

            let result = check_code(&app.code_input);

            match result {
                Ok(res) => {
                    login_state.neptun = res.to_owned();
                    if app.password_input.len() > 0 && app.code_input.len() == 6 {
                        
                        login_state.user = logger(&app.code_input, &app.password_input);

                        match login_state.user {
                            Some(_) => {
                                app.current_login_parameter = LoginHighlight::Password;
                                std::thread::sleep(std::time::Duration::from_secs(1));
                                app.current_screen = CurrentScreen::Home;
                            }
                            None => {
                                login_state.neptun = "Not Found!".to_owned();
                                app.current_login_parameter = LoginHighlight::Password;
                            }
                        }
                    } else {
                        app.current_login_parameter = LoginHighlight::Password;
                    }
                }
                Err(err) => {
                    login_state.neptun = err.to_owned();
                    login_state.password = login_state.password.replace("ERROR", "");
                    app.current_login_parameter = LoginHighlight::Password;
                }
            }
        }

        LoginHighlight::Password => {
            let result = check_password(&app.password_input);

            match result {
                Ok(res) => {
                    login_state.password = res.to_owned();
                    if app.code_input.len() == 6 && app.password_input.len() > 3 {
                        login_state.user = logger(&app.code_input, &app.password_input);

                        match login_state.user {
                            Some(_) => {
                                app.current_login_parameter = LoginHighlight::Neptun;
                                std::thread::sleep(std::time::Duration::from_secs(1));
                                app.current_screen = CurrentScreen::Home;
                            }
                            None => {
                                login_state.neptun = "Not Found!".to_owned();
                                app.current_login_parameter = LoginHighlight::Neptun;
                            }
                        }
                    } else {
                        app.current_login_parameter = LoginHighlight::Neptun;
                    }
                }
                Err(err) => {
                    login_state.password = err.to_owned();
                    login_state.neptun = login_state.neptun.replace("ERROR", "");
                    app.current_login_parameter = LoginHighlight::Neptun;
                }
            }
        }

        LoginHighlight::None => app.current_login_parameter = LoginHighlight::Neptun,
    }
}

pub fn handle_input(char: char, app: &mut App, login_state: &mut LoginState) {
    match app.current_login_parameter {
        LoginHighlight::Neptun => {
            app.code_input.push(char.to_ascii_uppercase());

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
            login_state.password = login_state.password.replace("ERROR", "");
            app.password_input.push(char);
        }
        LoginHighlight::None => {
            if char == 'q' {
                app.current_screen = CurrentScreen::Exiting
            };
        }
    }
}

pub fn handle_deletion(app: &mut App, login_state: &mut LoginState) {
    match app.current_login_parameter {
        LoginHighlight::Neptun => {
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
    }
}

pub fn handle_navigation(key: KeyCode) -> CurrentScreen {
    match key {
        KeyCode::Char('1') => CurrentScreen::Home,
        KeyCode::Char('2') => CurrentScreen::Courses,
        KeyCode::Char('3') => CurrentScreen::TimeTable,
        KeyCode::Char('q') => CurrentScreen::Exiting,
        _ => CurrentScreen::Home,
    }
}
