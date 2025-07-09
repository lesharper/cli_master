use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub fn render_utility_list(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .utilities
        .iter()
        .map(|util| ListItem::new(util.name()))
        .collect();

    let list = List::new(items)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title("Utilities".bold().green()),
        )
        .highlight_style(Style::new().yellow().bold())
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(Some(app.selected));
    frame.render_stateful_widget(list, area, &mut state);
}
