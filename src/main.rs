use tui::{widgets::Block, backend::Backend, Frame};
use std::io;
use tui::widgets::Borders;
mod components;
use tui::layout::Rect;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use components::{run::start, app::RuManga};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen ,EnableMouseCapture)?;
    let backend = tui::backend::CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    let _size = Rect::new(11, 234, 114, 18);
    terminal.draw(|f| {
        let _size = f.size();
        components::run::start(f);
        
    })?;
    
    Ok(())
}

