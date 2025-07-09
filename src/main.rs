// main.rs
use color_eyre::Result;

mod app;
mod ui;
mod util;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal)?;
    ratatui::restore();
    Ok(result)
}
