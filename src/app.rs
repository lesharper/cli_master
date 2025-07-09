use color_eyre::eyre::*;
// app.rs
use crate::{
    ui::render::render,
    util::{Utility, UtilityConfig},
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;
type Result<T> = color_eyre::eyre::Result<T, Error>;

#[derive(Debug)]
pub struct App {
    running: bool,
    pub selected: usize,
    pub utilities: Vec<Utility>,
    pub state: AppState,
}

#[derive(Debug)]
pub enum AppState {
    Selecting,
    Configuring(usize),
    Running(usize),
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
        }
    }

    pub fn run(mut self, mut terminal: ratatui::DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| render(frame, &self))?;
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
                }
                (_, KeyCode::Esc) | (KeyModifiers::CONTROL, KeyCode::Char('c' | 'C')) => {
                    self.quit();
                }
                _ => {}
            },
            AppState::Configuring(idx) => match (key.modifiers, key.code) {
                (_, KeyCode::Enter) => {
                    self.state = AppState::Running(idx);
                    self.run_utility(idx)?;
                }
                (_, KeyCode::Esc) => {
                    self.state = AppState::Selecting;
                }
                _ => {}
            },
            AppState::Running(_) => match (key.modifiers, key.code) {
                (_, KeyCode::Esc) => {
                    self.state = AppState::Selecting;
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn run_utility(&self, idx: usize) -> Result<()> {
        if let Some(utility) = self.utilities.get(idx) {
            // Заглушка для выполнения утилиты
            println!("Running: {}", utility.name());
        }
        Ok(())
    }
}
