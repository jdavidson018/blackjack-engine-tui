use ratatui::layout::Alignment;
use ratatui::prelude::Stylize;
use ratatui::{
    layout::Rect,
    style::Style,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui::style::Color;
use crate::constants::TITLE;

pub fn render_border(frame: &mut Frame, screen: Rect) {
    let border_block = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(Color::White));
    let border = Paragraph::new(Text::default())
        .alignment(Alignment::Center)
        .block(border_block);
    frame.render_widget(border, screen)
}

pub fn render_title_block(frame: &mut Frame, rect: Rect) {
    // Just a placeholder method, rendering may happen in a different
    // Impl section
    let title_paragraph = Paragraph::new(TITLE)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title_paragraph, rect)
}

pub fn render_sub_title_block(frame: &mut Frame, rect: Rect) {
    let sub_title = Paragraph::new("Made by Freeside Software")
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(sub_title, rect);
}



pub trait MenuNavigation {
    fn get_menu_length(&self) -> usize;
    fn get_menu_index(&self) -> i8;
    fn set_menu_index(&mut self, index: i8);

    fn increment_menu_index(&mut self, increment: i8) {
        let current_index = self.get_menu_index();
        if increment < 0 && self.get_menu_index() <= 0 {
            return
        }
        if increment > 0 && self.get_menu_index() >= (self.get_menu_length() - 1) as i8 {
            return
        }
        self.set_menu_index(current_index + increment);

    }
}
