use ratatui::{layout::{Constraint, Direction, Layout}, Frame};

use crate::app::{App,CurrentScreen};

pub fn ui(f: &mut Frame, app: &App){
    // Az alap sablont hozzuk létre itt ami 3 oszlopra részre osztunk
    let layout_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ]).split(f.size());
    
}
