use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, self, disable_raw_mode, LeaveAlternateScreen}, execute, event::{Event::{Key, self}, self, DisableMouseCapture, KeyCode, read, KeyEvent, KeyModifiers}, cursor::Show};
use tui::{terminal::Terminal, backend::CrosstermBackend, widgets::Wrap};
use tui::{Frame, backend::Backend, layout::{Layout, Direction}, text::Spans, widgets::Paragraph};
use std::{io::{self, stdout}, time::Instant};
use crate::components::app;
use tui::layout::Constraint;
use crate::components::app::RuManga;

use super::app::{AppTabs, ui, Mode, TrueTab};
pub fn start<B: Backend>(_f: &mut Frame<B>) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut frame = terminal.get_frame();
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
     */
    let stdouts = io::stdout();
    let backends = CrosstermBackend::new(stdouts);
    let mut terminals = Terminal::new(backends)?;

    run_app(&mut terminals, &mut frame);
 
    Ok(())
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, fr: &mut Frame<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;
        //read_spook(terminal);
        keys(terminal, fr);
        //read_keys(terminal,  fr, &mut RuManga::new())
    }
    
}
pub fn keys<B: Backend>(terminal: &mut Terminal<B>, f: &mut Frame<B>) {
    let mut ru_apps = RuManga::new();
    loop {
        ru_apps.tab();
    }
}


pub fn read_keys<B: Backend>(terminal: &mut Terminal<B>,f: &mut Frame<B>, ru_app: &mut RuManga) -> ! {
    let mut apps = app::TrueTab::new();
    loop {
        if let Ok(Event::Key(key)) = event::read() {
            match apps.index {
                0 => apps.next(),
                1 => apps.previous(),
                _ => {}
            }  
        };
    }
}

pub fn read_spook<B: Backend>(terminal: &mut Terminal<B>) -> KeyEvent {
    loop {
        if let Ok(event) = read() {
            if let Key(key_event) = event {
                return key_event;
            }
        } else {
            app_fail( terminal, "Killing process", false);

        }
    } 
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