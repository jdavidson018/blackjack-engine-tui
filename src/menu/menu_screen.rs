use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Alignment, Line};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph};
use crate::menu::menu_screen::MainMenuOption::{Continue, HighScores, Play, Settings, Tutorial};
use crate::model::{Model, ModelResponse};
use crate::ui::{render_border, render_sub_title_block, render_title_block, MenuNavigation};

enum MainMenuOption {
    Play,
    Continue,
    Tutorial,
    HighScores,
    Settings,
}

impl MainMenuOption {
    pub fn to_string(&self) -> String {
        match self {
            Play => "Play".to_string(),
            Continue => "Continue".to_string(),
            Tutorial => "Tutorial".to_string(),
            HighScores => "High Scores".to_string(),
            Settings => "Settings".to_string(),
        }
    }
}

const MAIN_MENU_ITEMS: [MainMenuOption; 5] = [
    Play,
    Continue,
    Tutorial,
    HighScores,
    Settings
];


pub struct MenuScreen {
    active_menu_index: i8,
}

impl MenuScreen {
    pub fn new() -> MenuScreen {
        MenuScreen {
            active_menu_index: 0,
        }
    }

    fn render_menu_body(&self, frame: &mut Frame, rect: Rect) {
        let mut menu_body: Vec<Line<'_>> = vec![];

        for (i, item) in MAIN_MENU_ITEMS.iter().enumerate() {
            menu_body.push(Line::from(""));
            let mut text = if self.active_menu_index == i as i8 {
                "> ".to_string()
            } else {
                String::new()
            };

            text.push_str(item.to_string().as_str());

            if self.active_menu_index == i as i8 {
                menu_body.push(Line::from(text).fg(Color::Green))
            } else {
                menu_body.push(Line::from(text));
            }
        }

        let menu_options = Paragraph::new(menu_body)
            .bold()
            .alignment(Alignment::Center)
            .block(Block::default());
        frame.render_widget(menu_options, rect);
    }
}

impl MenuNavigation for MenuScreen {
    fn get_menu_length(&self) -> usize {
        MAIN_MENU_ITEMS.len()
    }

    fn get_menu_index(&self) -> i8 {
        self.active_menu_index
    }

    fn set_menu_index(&mut self, index: i8) {
        self.active_menu_index = index
    }
}

impl Model for MenuScreen {
    fn update(&mut self) -> std::io::Result<ModelResponse> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                return Ok(ModelResponse::Refresh);
            }
            return match key.code {
                KeyCode::Char('q') => Ok(ModelResponse::Exit),
                // More cursor down
                KeyCode::Char('j') | KeyCode::Down => {
                    self.increment_menu_index(1);
                    return Ok(ModelResponse::Refresh);
                }
                // More cursor up
                KeyCode::Char('k') | KeyCode::Up => {
                    self.increment_menu_index(-1);
                    return Ok(ModelResponse::Refresh);
                }
                KeyCode::Enter => {
                    return Ok(ModelResponse::NavToSettings)
                }
                _ => Ok(ModelResponse::Refresh),
            }
        }
        Ok(ModelResponse::Refresh)
    }

    fn ui(&mut self, frame: &mut Frame) {
        // We will use the entire screen
        let screen = frame.area();
        render_border(frame, screen);

        // break the screen into chunks
        let menu_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(14),
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Ratio(2,5),
                Constraint::Ratio(1,5),
            ])
            .split(screen);

        render_title_block(frame, menu_layout[0]);
        render_sub_title_block(frame, menu_layout[1]);
        self.render_menu_body(frame, menu_layout[3]);
    }
}