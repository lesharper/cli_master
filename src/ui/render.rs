use crate::app::{App, AppState};
use figlet_rs::FIGfont;
use ratatui::{prelude::*, widgets::Paragraph};

use crate::ui::render_config::render_config;
use crate::ui::render_running::render_running;
use crate::ui::render_utility_list::render_utility_list;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(7), // Для заголовка
            Constraint::Length(3), // Для описания
            Constraint::Min(0),    // Для списка или конфигурации
        ])
        .split(frame.area());

    // Рендеринг заголовка
    let standard_font = FIGfont::standard().expect("Failed to load standard FIGfont");
    let figure = standard_font
        .convert("CLI MASTER")
        .expect("Failed to convert text to FIGfont");
    let title = Paragraph::new(figure.to_string()).bold().cyan().centered();
    frame.render_widget(title, chunks[0]);

    // Рендеринг описания
    let desc = Paragraph::new("Welcome to CLI_MASTER - Your Ultimate CLI Utility Aggregator!")
        .white()
        .centered();
    frame.render_widget(desc, chunks[1]);

    // Рендеринг в зависимости от состояния
    match app.state {
        AppState::Selecting => render_utility_list(frame, app, chunks[2]),
        AppState::Configuring(idx) => render_config(frame, app, idx, chunks[2]),
        AppState::Running(idx) => render_running(frame, app, idx, chunks[2]),
    }
}
