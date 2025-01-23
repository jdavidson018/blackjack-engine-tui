use std::collections::HashMap;
use crate::constants::MENU_ITEMS;

pub enum CurrentScreen {
    Menu,
    Play,
    Tutorial,
    Settings,
    HighScores,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub active_menu_index: i8,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Menu,
            active_menu_index: 0,
        }
    }

    pub fn increment_active_menu_index(&mut self, increment: i8) {
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

    pub fn set_current_screen(&mut self, desired_screen: CurrentScreen) {
        // when we switch screens, reset the manu index
        self.active_menu_index = 0;
        self.current_screen = desired_screen;
    }

    pub fn determine_current_screen(&mut self) {
        let selected_item = MENU_ITEMS.get(self.active_menu_index as usize).unwrap_or(&"High Scores");

        self.current_screen = match *selected_item {
            "Play" => CurrentScreen::Play,
            "Tutorial" => CurrentScreen::Tutorial,
            "Settings" => CurrentScreen::Settings,
            "High Scores" => CurrentScreen::HighScores,
            _ => CurrentScreen::Menu,
        };
    }

}