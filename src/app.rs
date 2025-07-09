// app.rs
use crate::ui::render as ui;
use crate::util::{Utility, UtilityConfig};
use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;

#[derive(Debug)]
pub struct App {
    running: bool,
    pub selected: usize,
    pub utilities: Vec<Utility>,
    pub state: AppState,
    pub input: String,          // Для ввода пути
    pub character_index: usize, // Позиция курсора
    pub input_mode: InputMode,  // Режим ввода
    pub scroll_index: usize,    // Для прокрутки вывода
}

#[derive(Debug)]
pub enum AppState {
    Selecting,
    Configuring(usize),
    Running(usize),
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            selected: 0,
            utilities: vec![
                Utility::new(
                    "GetDirTree",
                    "Displays directory tree structure",
                    UtilityConfig::default(),
                ),
                Utility::new(
                    "Network Scanner",
                    "Scans local network",
                    UtilityConfig::default(),
                ),
                Utility::new(
                    "System Monitor",
                    "Monitors system resources",
                    UtilityConfig::default(),
                ),
            ],
            state: AppState::Selecting,
            input: String::new(),
            character_index: 0,
            input_mode: InputMode::Normal,
            scroll_index: 0,
        }
    }

    pub fn run(mut self, mut terminal: ratatui::DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| ui::render(frame, &self))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        if let Event::Key(key) = crossterm::event::read()? {
            if key.kind == KeyEventKind::Press {
                self.on_key_event(key)?;
            }
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match self.state {
            AppState::Selecting => match (key.modifiers, key.code) {
                (_, KeyCode::Up) => {
                    self.selected = self.selected.saturating_sub(1);
                }
                (_, KeyCode::Down) => {
                    if self.selected < self.utilities.len().saturating_sub(1) {
                        self.selected += 1;
                    }
                }
                (_, KeyCode::Enter) => {
                    self.state = AppState::Configuring(self.selected);
                    if self.utilities[self.selected].name() == "GetDirTree" {
                        self.input_mode = InputMode::Editing;
                        self.input.clear();
                        self.character_index = 0;
                    }
                }
                (_, KeyCode::Esc) | (KeyModifiers::CONTROL, KeyCode::Char('c' | 'C')) => {
                    self.quit();
                }
                _ => {}
            },
            AppState::Configuring(idx) => {
                if self.utilities[idx].name() == "GetDirTree" {
                    match self.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('e') => {
                                self.input_mode = InputMode::Editing;
                            }
                            KeyCode::Esc => {
                                self.state = AppState::Selecting;
                                self.input.clear();
                                self.character_index = 0;
                            }
                            _ => {}
                        },
                        InputMode::Editing => match key.code {
                            KeyCode::Enter => {
                                if !self.input.is_empty() {
                                    self.utilities[idx].config.path = Some(self.input.clone());
                                    self.state = AppState::Running(idx);
                                    self.utilities[idx].run_get_dir_tree()?;
                                    self.input_mode = InputMode::Normal;
                                    self.scroll_index = 0;
                                }
                            }
                            KeyCode::Char(to_insert) => {
                                self.enter_char(to_insert);
                            }
                            KeyCode::Backspace => {
                                self.delete_char();
                            }
                            KeyCode::Left => {
                                self.move_cursor_left();
                            }
                            KeyCode::Right => {
                                self.move_cursor_right();
                            }
                            KeyCode::Esc => {
                                self.input_mode = InputMode::Normal;
                            }
                            _ => {}
                        },
                    }
                } else {
                    match (key.modifiers, key.code) {
                        (_, KeyCode::Enter) => {
                            self.state = AppState::Running(idx);
                            self.run_utility(idx)?;
                        }
                        (_, KeyCode::Esc) => {
                            self.state = AppState::Selecting;
                        }
                        _ => {}
                    }
                }
            }
            AppState::Running(idx) => match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('b')) => {
                    self.utilities[idx].copy_output_to_clipboard()?;
                    // Для отладки, чтобы убедиться, что копирование вызывается
                    println!("Attempted to copy to clipboard");
                }
                (_, KeyCode::Up) => {
                    self.scroll_index = self.scroll_index.saturating_sub(1);
                }
                (_, KeyCode::Down) => {
                    if let Some(util) = self.utilities.get(idx) {
                        if util.name() == "GetDirTree" {
                            let output_lines = util
                                .config()
                                .output
                                .as_ref()
                                .map(|s| s.lines().count())
                                .unwrap_or(0);
                            if self.scroll_index < output_lines.saturating_sub(1) {
                                self.scroll_index += 1;
                            }
                        }
                    }
                }
                (_, KeyCode::Esc) => {
                    self.state = AppState::Selecting;
                    self.input.clear();
                    self.character_index = 0;
                    self.input_mode = InputMode::Normal;
                    self.scroll_index = 0;
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn run_utility(&mut self, idx: usize) -> Result<()> {
        if let Some(utility) = self.utilities.get_mut(idx) {
            match utility.name() {
                "GetDirTree" => utility.run_get_dir_tree(),
                _ => {
                    println!("Running: {}", utility.name());
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }
}
