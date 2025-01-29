use std::fmt::format;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Stylize};
use ratatui::widgets::{Block, Paragraph};
use crate::constants::{MENU_ITEMS, SETTINGS_ITEMS, TITLE};
use crate::menu::menu_screen::MenuScreen;
use crate::model::{Model, ModelResponse};
use crate::ui::render_border;

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

    fn render_title_block(&self, frame: &mut Frame, rect: Rect) {
        // Just a placeholder method, rendering may happen in a different
        // Impl section
        let title_paragraph = Paragraph::new(TITLE)
            .alignment(Alignment::Center)
            .block(Block::default());
        frame.render_widget(title_paragraph, rect)
    }

    fn render_sub_title_block(&self, frame: &mut Frame, rect: Rect) {
        let sub_title = Paragraph::new("")
            .alignment(Alignment::Center)
            .block(Block::default());
        frame.render_widget(sub_title, rect);
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

            text.push_str(item);

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

    fn increment_active_menu_index(&mut self, increment: i8) {
        // return if attempting to decrement below first option
        if increment < 0 && self.active_menu_index <= 0 {
            return;
        }
        // return if attempting to increment beyond last option
        if increment > 0 && self.active_menu_index >= (SETTINGS_ITEMS.len() - 1) as i8 {
            return;
        }
        self.active_menu_index = self.active_menu_index + increment as i8;
    }

    fn increment_current_menu_item(&mut self, increment: i8) {
        if self.active_menu_index == 0 {
            if increment < 0 && self.number_of_decks_value < 2 {
                return;
            }
            self.number_of_decks_value += increment;
        } else if self.active_menu_index == 1 {
            if increment < 0 && self.number_of_players_value < 2 {
                return;
            }
            self.number_of_players_value += increment;
        }
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
                    self.increment_active_menu_index(1);
                    return Ok(ModelResponse::Refresh);
                }
                // More cursor up
                KeyCode::Char('k') | KeyCode::Up => {
                    self.increment_active_menu_index(-1);
                    return Ok(ModelResponse::Refresh);
                }
                // Return to the Menu
                KeyCode::Char('m') => {
                    self.increment_active_menu_index(-1);
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

        self.render_title_block(frame, menu_layout[0]);
        self.render_sub_title_block(frame, menu_layout[1]);
        self.render_menu_body(frame, menu_layout[3]);
    }
}