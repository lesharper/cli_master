use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_running(frame: &mut Frame, app: &App, idx: usize, area: Rect) {
    let util = app.utilities.get(idx).unwrap();
    let running = Paragraph::new(format!("Running: {}\nPress Esc to return", util.name()))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title("Running".bold().magenta()),
        )
        .white()
        .centered();

    frame.render_widget(running, area);
}
