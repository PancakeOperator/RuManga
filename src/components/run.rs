use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, self, disable_raw_mode, LeaveAlternateScreen}, execute, event::{Event, self, DisableMouseCapture}, cursor::Show};
use tui::{terminal::Terminal, backend::CrosstermBackend, widgets::Wrap};
use tui::{Frame, backend::Backend, layout::{Layout, Direction}, text::Spans, widgets::Paragraph};
use std::{io::{self, stdout}, time::Instant};
use crate::components::app;
use tui::layout::Constraint;
use crate::components::app::RuManga;

use super::app::{AppTabs, ui};
pub fn start<B: Backend>(f: &mut Frame<B>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut frame = terminal.get_frame();
    let rust_manga = RuManga::new();
    /*
    match rust_manga {
        _ => {
    
            ui(&mut frame, &mut RuManga::new());
        }
        _ => {
            app_fail(&mut terminal, "Fail", false);
        }
    }  */
    /*
    if let apps = rust_manga {
        app::ui(&mut frame, &mut RuManga::new())
    } else {
        app_fail(&mut terminal, "failure to start app", false);
    }
     */
    ui(f, &mut RuManga::new());

    
    Ok(())
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


        if instant {
            return;
        }

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                _ => {
                    return;
                }
            }
        }
    }
}