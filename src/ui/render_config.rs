use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_config(frame: &mut Frame, app: &App, idx: usize, area: Rect) {
    let util = app.utilities.get(idx).unwrap();
    let config = Paragraph::new(format!(
        "Configuring: {}\nPress Enter to run or Esc to cancel",
        util.name()
    ))
    .block(
        Block::new()
            .borders(Borders::ALL)
            .title("Configuration".bold().blue()),
    )
    .white()
    .centered();

    frame.render_widget(config, area);
}
