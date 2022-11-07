use std::fmt::Error;

use tui::{widgets::{Tabs, Block, Borders, Widget}, layout::{Rect, Constraint, Direction, Layout}, Frame, backend::Backend, style::{Style, Modifier, Color}, text::Spans};


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
enum Mode {
    InputMode,
    ViewMode,
}

#[derive(Clone)]
pub struct RuManga {
    login: Login,
    tabs: AppTabs,
    mode: Mode,
    search: String,
    search_fail: bool,
    exit: bool
}

impl RuManga {
    pub fn new() -> RuManga {
        return RuManga {
            login: Login::UserName,
            tabs: AppTabs::New,
            mode: Mode::ViewMode,
            search: String::new(),
            search_fail: false,
            exit: false,
        };
    }

    fn tab(&mut self) {
        self.tabs = match self.tabs {
            AppTabs::New => AppTabs::UpdateList,
            AppTabs::UpdateList => AppTabs::View,
            AppTabs::View => AppTabs::New,  
        };
    }

    fn search(&mut self) {
        match self.mode {
            Mode::ViewMode => {
                self.mode = Mode::InputMode;
                self.search_fail = false;
            }
            _ => {},
        }
        match self.tabs {
            AppTabs::New => {
                self.tabs = AppTabs::UpdateList;
            }
            AppTabs::New => {
                self.tabs = AppTabs::View;
            }
            AppTabs::UpdateList => {
                self.tabs = AppTabs::View;
            }
            AppTabs::UpdateList => {
                self.tabs = AppTabs::New;
            }
            AppTabs::View => {
                self.tabs = AppTabs::New;
            }
            AppTabs::View => {
                self.tabs = AppTabs::UpdateList;
            }
            _ => {},
        }
    }
    
    fn escape(&mut self) {
        match self.mode {
            Mode::InputMode => self.mode = Mode::ViewMode,
            _ => self.exit = true,
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, ru_app: &mut RuManga) {
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
        .select(match ru_app.tabs {
            AppTabs::New => 0,
            AppTabs::UpdateList => 3,
            AppTabs::View => 2,
        })
        .highlight_style(
            Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Green),
        );

        f.render_widget(tabs, sub_top_frame[0])
}
   

