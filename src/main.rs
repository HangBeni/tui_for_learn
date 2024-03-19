mod app;
mod ui;
use crossterm::{
    event::{self, EnableMouseCapture, Event},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    terminal, Terminal,
};

use crate::{
    app::{App, Course, CurrentScreen},
    ui::ui,
};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    //terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

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
                app::CurrentScreen::Home => {}
                app::CurrentScreen::Course => {}
                app::CurrentScreen::TimeTable => {}

            }
        }
    }
}
