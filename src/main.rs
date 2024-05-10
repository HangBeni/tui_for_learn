mod app;
mod ui;
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    terminal, Terminal,
};
use tui_for_learn::util::types::{Course, CurrentScreen};

use crate::{
    app::{App},
    ui::ui,
};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    //terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

//Az app folyamatát kezeli (lap váltás stb.)
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::Home => match key.code {
                    KeyCode::Char('2') => app.current_screen = CurrentScreen::Courses,
                    KeyCode::Char('3') => app.current_screen = CurrentScreen::TimeTable,
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                    _ => {}
                },
                CurrentScreen::Courses => match key.code {
                    KeyCode::Char('1') => app.current_screen = CurrentScreen::Home,
                    KeyCode::Char('3') => app.current_screen = CurrentScreen::TimeTable,
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                    _ => {}
                },
                CurrentScreen::TimeTable => match key.code {
                    KeyCode::Char('1') => app.current_screen = CurrentScreen::Home,
                    KeyCode::Char('2') => app.current_screen = CurrentScreen::Courses,
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
            }
        }
    }
}
