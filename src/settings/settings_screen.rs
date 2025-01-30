use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Stylize};
use ratatui::widgets::{Block, Paragraph};
use crate::model::{Model, ModelResponse};
use crate::settings::settings_screen::SettingsMenuOption::{NumberOfDecks, NumberOfPlayers};
use crate::ui::{render_border, render_sub_title_block, render_title_block, MenuNavigation};

enum SettingsMenuOption {
    NumberOfDecks,
    NumberOfPlayers
}

impl SettingsMenuOption {
    pub fn to_string(&self) -> String {
        match self {
            NumberOfDecks => "Number of Decks".to_string(),
            NumberOfPlayers => "# of Players".to_string()
        }
    }
}

const SETTINGS_ITEMS: [SettingsMenuOption; 2] = [
    NumberOfDecks,
    NumberOfPlayers
];

pub struct SettingsScreen {
    active_menu_index: i8,
    number_of_decks_value: i8,
    number_of_players_value: i8,
}

impl SettingsScreen {
    pub fn new() -> SettingsScreen {
        SettingsScreen {
            active_menu_index: 0,
            number_of_decks_value: 6,
            number_of_players_value: 1, //TODO: Default should be a full table
        }
    }

    fn render_menu_body(&self, frame: &mut Frame, rect: Rect) {
        let mut menu_body: Vec<Line<'_>> = vec![];

        for (i, item) in SETTINGS_ITEMS.iter().enumerate() {
            menu_body.push(Line::from(""));
            let mut text = if self.active_menu_index == i as i8 {
                "> ".to_string()
            } else {
                String::new()
            };

            text.push_str(item.to_string().as_str());

            if i == 0 {
                text.push_str(format!(": < {} >", self.number_of_decks_value).as_str());
            } else if i == 1 {
                text.push_str(format!(": < {} >", self.number_of_players_value).as_str());
            }

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

    fn increment_current_menu_item(&mut self, increment: i8) {
        let menu_item = SETTINGS_ITEMS.get(self.active_menu_index as usize).unwrap();
        match menu_item {
            NumberOfDecks => {
                if increment < 0 && self.number_of_decks_value < 2 {
                    return;
                }
                self.number_of_decks_value += increment;
            }
            NumberOfPlayers => {
                if increment < 0 && self.number_of_players_value < 2 {
                    return;
                }
                self.number_of_players_value += increment;
            }
        }
    }
}

impl MenuNavigation for SettingsScreen {
    fn get_menu_length(&self) -> usize {
        SETTINGS_ITEMS.len()
    }

    fn get_menu_index(&self) -> i8 {
        self.active_menu_index
    }

    fn set_menu_index(&mut self, index: i8) {
        self.active_menu_index = index
    }
}

impl Model for SettingsScreen {
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
                // Return to the Menu
                KeyCode::Char('m') => {
                    return Ok(ModelResponse::NavToMainMenu);
                }
                // Increment current value up
                KeyCode::Char('l') | KeyCode::Right => {
                    self.increment_current_menu_item(1);
                    return Ok(ModelResponse::Refresh);
                }
                // Increment current value down
                KeyCode::Char('h') | KeyCode::Left => {
                    self.increment_current_menu_item(-1);
                    return Ok(ModelResponse::Refresh);
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
                Constraint::Ratio(1,5),
                Constraint::Ratio(1,5),
                Constraint::Ratio(1,5)
            ])
            .split(screen);

        render_title_block(frame, menu_layout[0]);
        render_sub_title_block(frame, menu_layout[1]);
        self.render_menu_body(frame, menu_layout[3]);
    }
}