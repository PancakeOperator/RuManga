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
    //for the frame
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen ,EnableMouseCapture)?;
    let backend = tui::backend::CrosstermBackend::new(stdout);
    let mut terminal = tui::Terminal::new(backend)?;
    let mut frame = terminal.get_frame();

    //for the terminal
    let mut stdouts = io::stdout();
    let backends = tui::backend::CrosstermBackend::new(stdouts);
    let mut terminals = tui::Terminal::new(backends)?;


    components::run::run_app(&mut terminals, &mut frame);
    
    
    Ok(())
}

