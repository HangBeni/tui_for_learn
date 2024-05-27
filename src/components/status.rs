use ratatui::text::Line;

pub fn status_bar<'a>() -> Line<'a> {
    let status_bar: Line = vec![
        "Location".into(),
        "|".into(),
        "Command status".into(),
        "|".into(),
        "Current Time".into(),
    ]
    .into(); 
status_bar
}