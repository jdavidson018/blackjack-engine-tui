use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use ratatui::crossterm::cursor::SetCursorStyle::SteadyBar;
use ratatui::layout::Alignment;
use ratatui::prelude::Stylize;
use ratatui::style::Color::Green;
use ratatui::symbols::Marker;
use ratatui::widgets::canvas;
use ratatui::widgets::canvas::Canvas;
use crate::app::{App, CurrentScreen};
use crate::constants::{MENU_ITEMS, SETTINGS_ITEMS, TITLE};
use crate::menu::menu_screen::BORDER;
use crate::playing_card::PlayingCard;

// Renders a border around the entire Rect passed in
pub fn render_border(frame: &mut Frame, screen: Rect) {
    let border_block = Block::default()
        .borders(Borders::all())
        .style(Style::default().fg(BORDER));
    let border = Paragraph::new(Text::default())
        .alignment(Alignment::Center)
        .block(border_block);
    frame.render_widget(border, screen)
}
pub fn ui(frame: &mut Frame, app: &App) {
    let main_area = frame.area();
    match app.current_screen {
        CurrentScreen::Menu => render_menu_ui(frame, app, main_area),
        CurrentScreen::Play => render_canvas(frame, app, main_area),
        CurrentScreen::Tutorial => screen_render_placeholder(frame, app, main_area, "Tutorial"),
        CurrentScreen::HighScores => screen_render_placeholder(frame, app, main_area, "High Scores"),
        CurrentScreen::Settings => screen_render_placeholder(frame, app, main_area, "Settings"), // Handle other screens if needed
    }
}

fn screen_render_placeholder(frame: &mut Frame, app: &App, main_area: Rect, title: &str) {
    let menu_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(14),
                Constraint::Min(1),
            ].as_ref()
        )
        .split(main_area);

    // Title
    let title_paragraph = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title_paragraph, menu_layout[0]);
}

fn render_menu_ui(frame: &mut Frame, app: &App, main_area: Rect) {
    let menu_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(14),
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
            ].as_ref()
        )
        .split(main_area);

    // Title
    let title_paragraph = Paragraph::new(TITLE)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title_paragraph, menu_layout[0]);

    let sub_title = Paragraph::new("Made by Freeside Software")
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(sub_title, menu_layout[1]);

    let mut menu_body: Vec<Line<'_>> = vec![];

    for (i, item) in MENU_ITEMS.iter().enumerate() {
        menu_body.push(Line::from(""));
        let mut text = if app.active_menu_index == i as i8 {
            "> ".to_string()
        } else {
            String::new()
        };

        text.push_str(item);

        if app.active_menu_index == i as i8 {
            menu_body.push(Line::from(text).fg(Green))
        } else {
            menu_body.push(Line::from(text));
        }
    }

    let menu_options = Paragraph::new(menu_body)
        .bold()
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(menu_options, menu_layout[3]);
}

fn render_canvas(frame: &mut Frame, app: &App, main_area: Rect) {
    let canvas = Canvas::default()
        .block(Block::bordered().title("  Live Play  ").bg(Color::White))
        .marker(Marker::Braille)
        .paint(|context| {

            context.draw(&canvas::Rectangle {
                x: 0.0,
                y: 0.0,
                width: main_area.width as f64,
                height: main_area.height as f64,
                color: Color::White,
            });

            let ace_spades = PlayingCard {
                x: 54.0,
                y: 40.0,
                width: 4.0,
                height: 3.0,
                symbol: '♠',
                color: Color::Black,
                value: "A".to_string(),
            };
            ace_spades.render(context);

            let king_hearts = PlayingCard {
                x: 60.0,
                y: 40.0,
                width: 4.0,
                height: 3.0,
                symbol: '♥',
                color: Color::Red,
                value: "K".to_string(),
            };
            king_hearts.render(context);

            // You can also print multiple lines of text
            context.print(
                10.0,
                10.0,
                format!("Score: {}", 42),  // Example with dynamic text
            );
        })
        .x_bounds([0.0, main_area.width as f64])
        .y_bounds([0.0, main_area.height as f64]);

    frame.render_widget(canvas, main_area);
}