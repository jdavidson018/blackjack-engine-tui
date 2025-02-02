use ratatui::layout::Alignment;
use ratatui::{
    layout::Rect,
    style::Style,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui::prelude::{Line, Span};
use ratatui::style::Color;
use ratatui::widgets::Wrap;
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

pub fn render_text(frame: &mut Frame, rect: Rect, text: &str) {
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(paragraph, rect);
}

pub fn render_bottom_text(frame: &mut Frame, rect: Rect, text: &str) {
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Left)  // Changed to Left alignment
        .block(Block::default())
        .wrap(Wrap { trim: true })   // Enable text wrapping
        .scroll((0, 0));            // Start from top-left

    // Create a new rect for bottom alignment
    let bottom_rect = Rect {
        x: rect.x,
        y: rect.y + rect.height.saturating_sub(1), // Move to bottom, leaving 1 line
        width: rect.width,
        height: 1,  // Only use 1 line of height
    };

    frame.render_widget(paragraph, bottom_rect);
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

pub fn render_footer_spans(frame: &mut Frame, spans: Vec<String>, rect: Rect) {
    let mut spans = vec![" Q ".to_string(), " Quit ".to_string(),
    " M ".to_string(), " Menu ".to_string()];

    let styles = [
        Style::default().bg(Color::Gray).fg(Color::DarkGray),
        Style::default().fg(Color::DarkGray),
    ];

    frame.render_widget(
        Line::from(
            spans
                .iter()
                .enumerate()
                .map(|(idx, content)| Span::styled(content, styles[idx % 2]))
                .collect::<Vec<_>>(),
        ).left_aligned(),
        rect
    );
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
