use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, self, disable_raw_mode, LeaveAlternateScreen}, execute, event::{Event::{Key, self}, self, DisableMouseCapture, KeyCode, read, KeyEvent, KeyModifiers}, cursor::Show};
use tui::{terminal::Terminal, backend::CrosstermBackend, widgets::Wrap};
use tui::{Frame, backend::Backend, layout::{Layout, Direction}, text::Spans, widgets::Paragraph};
use std::{io::{self, stdout}, time::Instant};
use crate::components::app;
use tui::layout::Constraint;
use errno::errno;
use crate::components::app::RuManga;

use super::app::{AppTabs, ui, Mode};
pub fn start<B: Backend>(f: &mut Frame<B>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut frame = terminal.get_frame();

    run_app(&mut terminal);
 
    Ok(())
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;
        let mut ru_app = RuManga::new();
        let mut tab_start = AppTabs::View;
        let mut update = AppTabs::UpdateList;
        if let Event::Key(key) = event::read()? {
            match ru_app.tabs {
                AppTabs::UpdateList => todo!(),
                AppTabs::View => match ru_app.mode {
                    Mode::ViewMode => match key.code {
                        KeyCode::Char('q') => {
                            die("\n Ending Program!");
                        }
                        KeyCode::Tab => {
                            ru_app.tab();
                        }
                        KeyCode::Esc => {
                            ru_app.escape()
                        }
                        _ => {
                        }
                    }
                    Mode::InputMode => match key.code {
                        KeyCode::Char('q') => {
                            die("\nEnding Program")
                        }
                        KeyCode::Esc => {
                            ru_app.escape()
                        }
                        KeyCode::Tab => {
                            ru_app.tab()
                        }
                        KeyCode::Char('s') => {
                            ru_app.search()
                        }
                        _ => {}
                    }
                },
            }
        }
        /* 
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }


        }*/
    }
    
}

pub fn die<S: Into<String>>( msg: S) {
    let _ = terminal::disable_raw_mode();
    eprintln!("\n{}, {}", msg.into(), errno());
    std::process::exit(1);
}

pub fn app_fail<B: Backend>(terminal: &mut Terminal<B> ,msg: &str, instant: bool) {
    loop {
        let fail = terminal.draw(|f| {
            let size = f.size();
            let layout = Layout::default()
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .direction(Direction::Vertical)
                .split(size);
            let text = Spans::from(msg);
            let paragraph = Paragraph::new(text.clone()).wrap(Wrap {trim: true});

            f.render_widget(paragraph, layout[0]);
        });
    }
}