use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, self, disable_raw_mode, LeaveAlternateScreen}, execute, event::{Event::{Key, self}, self, DisableMouseCapture, KeyCode, read, KeyEvent, KeyModifiers}, cursor::Show};
use tui::{terminal::Terminal, backend::CrosstermBackend, widgets::Wrap};
use tui::{Frame, backend::Backend, layout::{Layout, Direction}, text::Spans, widgets::Paragraph};
use std::{io::{self, stdout}, time::Instant};
use crate::components::app;
use tui::layout::Constraint;
use crate::components::app::RuManga;

use super::app::{AppTabs, ui, Mode};
pub fn start<B: Backend>(f: &mut Frame<B>) -> Result<(), io::Error> {
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

    run_app(&mut terminal);
 
    Ok(())
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
    
}
/* 
pub fn read_keys<B: Backend>(terminal: &mut Terminal<B>,f: &mut Frame<B>, ru_app: &mut RuManga) {
    loop {
        terminal.draw(ui)?;
        if let Ok(Event::Key(key)) = event::read() {
            match ru_app.tabs {
                AppTabs::New => match ru_app.mode {
                    Mode::ViewMode => match key.code {
                        KeyCode::Char('q') => {
                            return;
                        }
                        KeyCode::Char('s') => {
                            ru_app.search();
                        }
                        KeyCode::Esc => {
                            ru_app.escape();
                        }
                        KeyCode::Tab => {
                            ru_app.tab();
                        }
                        _ => {}
                    }
                    Mode::InputMode => match key.code {
                        KeyCode::Esc => {
                            ru_app.escape();
                        }
                        KeyCode::Backspace => {
                            ru_app.search.pop();
                        }
                        KeyCode::Char(c) => {
                            ru_app.search.push(c)
                        }
                        _ => {}
                    },
                    
                }
                AppTabs::UpdateList => match ru_app.mode {
                    Mode::ViewMode => match key.code {
                        KeyCode::Char('q') => {
                            return;
                        }
                        KeyCode::Char('s') => {
                            ru_app.search();
                        }
                        KeyCode::Tab => {
                            ru_app.tab();
                        }
                        _ => {}
                    }
                    Mode::InputMode => match key.code {
                        KeyCode::Char(c) => {
                            ru_app.search.push(c);
                        }
                        KeyCode::BackTab => {
                            ru_app.search.pop();
                        }
                        KeyCode::Esc => {
                            ru_app.escape();
                        }
                        _ => {}
                    }
                },
                AppTabs::View => todo!(),
            }
        }
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
*/


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