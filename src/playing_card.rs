use ratatui::style::Color;
use ratatui::widgets::canvas;
use ratatui::widgets::canvas::{Shape, Rectangle, Painter, Context};

pub struct PlayingCard {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub symbol: char,
    pub color: Color,
    pub value: String,
}

impl PlayingCard {
    pub fn render(&self, context: &mut Context) {

        context.draw(&canvas::Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            color: self.color,
        });

        let text = format!("{}{}", self.value, self.symbol);
        let text_clone = text.clone();

        context.print(
            self.x,
            self.y + self.height - 1.0,
            text,
        );

        context.print(
            self.x + self.width - 1.0,
            self.y - 1.0,
            text_clone,
        );
    }
}
impl Default for PlayingCard {
    fn default() -> Self {
        PlayingCard {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 15.0,
            symbol: '♠',
            color: Color::White,
            value: "A".to_string(),
        }
    }
}