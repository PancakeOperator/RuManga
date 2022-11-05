use std::fmt::Error;

use tui::{widgets::{Tabs, Block, Borders, Widget}, layout::{Rect, Constraint, Direction, Layout}, Frame, backend::Backend, style::Style};


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

pub fn ui<B: Backend>(f: &mut Frame<B>) {

    let size = f.size();
    let vertical_chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints(
				[
					Constraint::Ratio(1, 3),
					Constraint::Ratio(1, 3),
					Constraint::Ratio(1, 3),
				]
				.as_ref(),
			)
			.split(size);
    let blocks = Block::default()
        .title("1 - 2")
        .border_style(Style::default())
        .borders(Borders::ALL);
    /*
    let block2 = Block::default()
        .title("2")
        .border_style(Style::default())
        .borders(Borders::ALL);
             */
    f.render_widget(blocks, vertical_chunks[1]);
    //f.render_widget(block2, vertical_chunks[1]);

}

