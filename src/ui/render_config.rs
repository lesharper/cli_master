use crate::app::{App, InputMode};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_config(frame: &mut Frame, app: &App, idx: usize, area: Rect) {
    let util = app.utilities.get(idx).unwrap();
    if util.name() == "GetDirTree" {
        let (msg, style) = match app.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "e".bold(),
                    " to start editing, ".into(),
                    "Esc".bold(),
                    " to cancel".into(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Enter".bold(),
                    " to submit, ".into(),
                    "Esc".bold(),
                    " to stop editing".into(),
                ],
                Style::default(),
            ),
        };
        let help_message = Paragraph::new(Text::from(Line::from(msg)).patch_style(style));
        let input_area =
            Layout::vertical([Constraint::Length(1), Constraint::Length(3)]).split(area)[1];
        let help_area =
            Layout::vertical([Constraint::Length(1), Constraint::Length(3)]).split(area)[0];
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(app.input.as_str())
            .style(match app.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Enter absolute path (e.g., /home/user/project)"));
        frame.render_widget(input, input_area);

        if let InputMode::Editing = app.input_mode {
            frame.set_cursor_position(Position::new(
                input_area.x + app.character_index as u16 + 1,
                input_area.y + 1,
            ));
        }
    } else {
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
}
