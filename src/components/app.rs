use std::{fmt::Error, io};

use crossterm::event::{KeyCode, Event, self};
use tui::{widgets::{Tabs, Block, Borders, Widget, Paragraph}, layout::{Rect, Constraint, Direction, Layout}, Frame, backend::Backend, style::{Style, Modifier, Color}, text::Spans, Terminal};

use super::run::app_fail;

#[derive(Clone)]
pub struct TrueTab<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize
}


impl<'a> TrueTab<'a> {
    pub fn new() -> TrueTab<'a> {
        TrueTab {
            titles: vec!["Tab0", "Tab1", "Tab2", "Tab3"],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
#[derive(Clone)]
pub enum AppTabs {
    New,
    UpdateList,
    View,
}

#[derive(Clone)]
pub enum Login {
    //may be required to use MangaDex
    UserName,
    //will be on host computer so should not need
    //to hash or salt? maybe
    UserPasscode,
}
#[derive(Clone)]
pub enum Mode {
    InputMode,
    ViewMode,
}

#[derive(Clone)]
pub struct RuManga<'a> {
    login: Login,
    pub tabs: TrueTab<'a>,
    pub mode: Mode,
    pub search: String,
    search_fail: bool,
    search_fail_msg: String,
    exit: bool
}

impl RuManga<'_> {
    pub fn escape(&mut self) {
        match self.mode {
            Mode::InputMode => self.mode = Mode::ViewMode,
            _ => self.exit = true,
        }
    }

    pub fn new() -> RuManga<'static> {
        return RuManga {
            login: Login::UserName,
            tabs: TrueTab::new(),
            mode: Mode::ViewMode,
            search: String::new(),
            search_fail: false,
            search_fail_msg: String::new(),
            exit: false,
        };
    }
    
    pub fn search(&mut self) {
        match self.mode {
            Mode::ViewMode => {
                self.mode = Mode::InputMode;
                self.search_fail = false;
            }
            _ => {},
        }
    }
    
    pub fn tab(&mut self)  {
        let mut app = TrueTab::new();
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Char('q') => return {},
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                _ => {},
            }
        }
        
    }

    
}



pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let app = TrueTab::new();
    let ru_app = RuManga::new();
    let main_frame = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let sub_top_frame = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(main_frame[0]);
    
    let tab_names = vec![
        Spans::from("New"), 
        Spans::from("UpdateList"), 
        Spans::from("View")];
    let block_navigation = Block::default().title("Navigation").borders(Borders::ALL);
    let tabs = Tabs::new(tab_names)
        .block(block_navigation)
        .select(
            app.index
        )
        .highlight_style(
            Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Green),
        );

        f.render_widget(tabs, sub_top_frame[0]);


        let block_search = Block::default()
            .title(match ru_app.search_fail {
                false => "fr".as_ref(),
                true => ru_app.search_fail_msg.as_ref(),
            })
            .borders(Borders::ALL);
        let search_func = Paragraph::new(ru_app.search.as_ref())
            .style(match ru_app.mode {
                Mode::InputMode => Style::default().fg(Color::LightBlue),
                Mode::ViewMode => match ru_app.search_fail {
                    true => Style::default().fg(Color::Red),
                    false => Style::default(),
                },
            })
            .block(block_search);

        f.render_widget(search_func, sub_top_frame[1]);
}
   

