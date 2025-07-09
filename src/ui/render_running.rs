use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn render_running(frame: &mut Frame, app: &App, idx: usize, area: Rect) {
    let util = app.utilities.get(idx).unwrap();
    let chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).split(area);

    if util.name() == "GetDirTree" {
        let output = util
            .config()
            .output
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("No output generated");
        let items: Vec<ListItem> = output.lines().map(|line| ListItem::new(line)).collect();
        let list = List::new(items)
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Directory Tree".bold().magenta()),
            )
            .style(Style::new().white())
            .highlight_style(Style::new().fg(Color::LightGreen).bold()) // Подсветка зеленым
            .highlight_symbol("> "); // Символ для выделенной строки
        let mut state = ListState::default();
        state.select(Some(app.scroll_index));
        frame.render_stateful_widget(list, chunks[0], &mut state);

        let instructions = Paragraph::new("Press Ctrl+B to copy to clipboard or Esc to return")
            .style(Style::new().white())
            .centered();
        frame.render_widget(instructions, chunks[1]);
    } else {
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
}
