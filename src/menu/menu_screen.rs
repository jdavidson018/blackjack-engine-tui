use std::ptr::from_ref;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Alignment, Line};
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, Paragraph};
use crate::constants::{MENU_ITEMS, TITLE};
use crate::model::{Model, ModelResponse};
use crate::ui::render_border;

pub const BORDER: Color = Color::White;

const CONTROLS_LIST: [&str; 8] = [
    "One", "two",
    "Three", "four",
    "Five", "six",
    "Seven", "eight",
];

pub struct MenuScreen {
    size_ok: bool, // I don't think I will need this
    active_menu_index: i8,
}

impl MenuScreen {
    pub fn new() -> MenuScreen {
        MenuScreen {
            size_ok: false,
            active_menu_index: 0,
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
        let sub_title = Paragraph::new("Made by Freeside Software")
            .alignment(Alignment::Center)
            .block(Block::default());
        frame.render_widget(sub_title, rect);
    }

    fn render_menu_body(&self, frame: &mut Frame, rect: Rect) {
        let mut menu_body: Vec<Line<'_>> = vec![];

        for (i, item) in MENU_ITEMS.iter().enumerate() {
            menu_body.push(Line::from(""));
            let mut text = if self.active_menu_index == i as i8 {
                "> ".to_string()
            } else {
                String::new()
            };

            text.push_str(item);

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
        if increment > 0 && self.active_menu_index >= (MENU_ITEMS.len() - 1) as i8 {
            return;
        }
        self.active_menu_index = self.active_menu_index + increment as i8;
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
                    self.increment_active_menu_index(1);
                    return Ok(ModelResponse::Refresh);
                }
                // More cursor up
                KeyCode::Char('k') | KeyCode::Up => {
                    self.increment_active_menu_index(-1);
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