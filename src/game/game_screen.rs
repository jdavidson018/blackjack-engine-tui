use std::arch::aarch64::vreinterpret_u8_f32;
use std::rc::Rc;
use std::time::Duration;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use blackjack_engine::game::{Game, GameAction, GameState};
use blackjack_engine::game_settings::GameSettings;
use blackjack_engine::hand::Hand;
use crate::model::{Model, ModelResponse};
use crate::ui::{render_border, render_bottom_right_text, render_bottom_text, render_footer_spans, render_text};

pub struct GameScreen {
    dealer_name: String,
    dealer_message: String,
    input_prompt: String,
    cursor_string: String,
    user_bet: f64,
    game: Game,
    bankroll: f64,
}

// Layout-related functions
impl GameScreen {
    fn create_main_layout(screen: Rect) ->  Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),              // Top margin
                Constraint::Ratio(4, 10),           // Dealer area
                Constraint::Ratio(4, 10),           // Player area
                Constraint::Ratio(1, 10),           // Stats/bankroll area
                Constraint::Length(1),              // Footer
            ])
            .split(screen)
    }

    fn create_dealer_section(dealer_area: Rect) -> (Rc<[Rect]>, Rc<[Rect]>) {
        Self::create_default_playable_section(dealer_area)
    }

    fn create_player_section(player_area: Rect) -> (Rc<[Rect]>, Rc<[Rect]>) {
        Self::create_default_playable_section(player_area)
    }

    fn create_default_playable_section(playable_area: Rect) -> (Rc<[Rect]>, Rc<[Rect]>) {
        let horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Ratio(2, 10),
                    Constraint::Ratio(6, 10),
                    Constraint::Ratio(2, 10),
                ])
                .split(playable_area);

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Ratio(3, 10),
                Constraint::Ratio(3, 10),
                Constraint::Ratio(3, 10),
            ])
            .split(horizontal[1]);

        (horizontal, vertical)
    }



    fn create_stats_section(stats_area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 20),           // Left margin
                Constraint::Ratio(6, 20),           // Stats 1
                Constraint::Ratio(6, 20),           // Stats 2
                Constraint::Ratio(6, 20),           // Stats 3
                Constraint::Ratio(1, 20),           // Right margin
            ])
            .split(stats_area)
    }

    fn create_footer_section(footer_area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),              // Left margin
                Constraint::Min(10),                // Footer content
                Constraint::Length(1),              // Right margin
            ])
            .split(footer_area)
    }
}

// Rendering-related functions
impl GameScreen {
    fn render_dealer_section(&self, frame: &mut Frame, dealer_wrapper: Rc<[Rect]>, dealer_rect: Rc<[Rect]>) {
        render_border(frame, dealer_wrapper[1]);
        render_text(frame, dealer_wrapper[1], " Dealer ");
        render_text(frame, dealer_rect[2], &self.dealer_message);
    }

    fn render_player_section(&self, frame: &mut Frame, player_wrapper: Rc<[Rect]>, player_rect: Rc<[Rect]>) {
        render_border(frame, player_wrapper[1]);
        render_text(frame, player_wrapper[1], " Jack ");
        render_bottom_text(frame, player_wrapper[1], format!(" Bet: ${} ", self.user_bet.to_string()).as_str());
        render_bottom_right_text(frame, player_wrapper[1], format!(" Bank: ${} ", self.bankroll.to_string()).as_str());

        self.render_player_hands(frame, player_rect);
    }

    fn render_player_hands(&self, frame: &mut Frame, player_vertical: Rc<[Rect]>) {
        match (*self.game.get_state()).clone() {
            GameState::WaitingForBet { .. } => {
                render_text(frame, player_vertical[1],
                            format!("{}{}{}",
                                    self.input_prompt,
                                    if self.user_bet == 0f64 { String::new() } else { self.user_bet.to_string() },
                                    self.cursor_string,
                            ).as_str()
                );
            },
            GameState::PlayerTurn { player_hands, active_hand_index, .. } => {
                if player_hands.is_empty() {
                    render_text(frame, player_vertical[1], "No Cards");
                    return;
                }

                for (i, hand) in player_hands.iter().enumerate() {
                    let hand_text = match &hand.outcome {
                        Some(outcome) => {
                            if i == active_hand_index {
                                format!("Cards {} - {} <", hand.to_string(), outcome.to_string().as_str())
                            } else {
                                format!("Cards {} - {}", hand.to_string(), outcome.to_string().as_str())
                            }
                        },
                        None => {
                            if i == active_hand_index {
                                format!("Cards {} <", hand.to_string())
                            } else {
                                format!("Cards {}", hand.to_string())
                            }
                        },
                    };
                    render_text(frame, player_vertical[i + 1], &hand_text);
                }
            },
            GameState::DealerTurn { player_hands, ..} | GameState::RoundComplete {player_hands, ..}=> {
                if player_hands.is_empty() {
                    render_text(frame, player_vertical[1], "No Cards");
                    return;
                }

                for (i, hand) in player_hands.iter().enumerate() {
                    let hand_text = match &hand.outcome {
                        Some(outcome) => {
                            format!("Cards {} - {}", hand.to_string(), outcome.to_string().as_str())
                        },
                        None => {
                            format!("Cards {}", hand.to_string())
                        },
                    };
                    render_text(frame, player_vertical[i + 1], &hand_text);
                }
            }
            _ => {}
        }
    }

    fn render_stats_section(&self, frame: &mut Frame, stats_rects: Rc<[Rect]>) {
        for rect in &stats_rects[0..=4] {
            render_border(frame, *rect);
        }
    }
}
impl GameScreen {
    pub fn new() -> GameScreen {
        let mut game = Game::new(
            GameSettings::new("Jack".to_string(), 6)
        );
        game.shuffle_shoe();
        GameScreen {
            dealer_name: String::from("Dealer McGee"),
            dealer_message: String::from("PLACE YOUR BET"),
            input_prompt: String::from("BET: $"),
            cursor_string: String::from("█"),
            user_bet: 0f64,
            game,
            bankroll: 0f64,
        }
    }

    pub fn handle_waiting_for_bet(&mut self, bankroll: f64) -> std::io::Result<ModelResponse> {
        self.bankroll = bankroll;
        self.dealer_message = "PLACE YOUR BET".to_string();
        self.input_prompt = "BET: $".to_string();
        match self.cursor_string.as_str() {
            "█" => self.cursor_string = String::from(" "),
            " " => self.cursor_string = String::from("█"),
            _ => {}
        }

        // Poll for events with a timeout of 500ms
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    return Ok(ModelResponse::Refresh);
                }
                return match key.code {
                    KeyCode::Char('0')
                    | KeyCode::Char('1')
                    | KeyCode::Char('2')
                    | KeyCode::Char('3')
                    | KeyCode::Char('4')
                    | KeyCode::Char('5')
                    | KeyCode::Char('6')
                    | KeyCode::Char('7')
                    | KeyCode::Char('8')
                    | KeyCode::Char('9') => {
                        if let KeyCode::Char(ch) = key.code {
                            // Convert char to digit and multiply existing bet by 10
                            if let Some(digit) = ch.to_digit(10) {
                                self.user_bet = self.user_bet * 10.0 + digit as f64;
                            }
                        }
                        Ok(ModelResponse::Refresh)
                    }
                    KeyCode::Enter => {
                        self.game.accept_user_bet(self.user_bet);
                        Ok(ModelResponse::Refresh)
                    },
                    KeyCode::Char('m') | KeyCode::Up => {
                        Ok(ModelResponse::NavToMainMenu)
                    }
                    KeyCode::Char('q') | KeyCode::Up => {
                        Ok(ModelResponse::Exit)
                    }
                    _ => Ok(ModelResponse::Refresh),
                }
            }
        }
        Ok(ModelResponse::Refresh)
    }

    pub fn handle_waiting_to_deal(&mut self, bankroll: f64, bet: f64) -> std::io::Result<ModelResponse> {
        self.dealer_message = "DEALING...".to_string();
        self.input_prompt = "".to_string();
        self.cursor_string = "".to_string();
        self.game.deal_initial_cards();

        Ok(ModelResponse::Refresh)
    }

    pub fn handle_player_turn(&mut self)  -> std::io::Result<ModelResponse> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                return Ok(ModelResponse::Refresh);
            }
            match key.code {
                KeyCode::Char('h') => {
                    self.game.process_player_action(GameAction::Hit, 0);
                },
                KeyCode::Char('s') => {
                    self.game.process_player_action(GameAction::Stand, 0);
                },
                KeyCode::Char('d') => {
                    self.game.process_player_action(GameAction::Double, 0);
                },
                KeyCode::Char('p') => {
                    self.game.process_player_action(GameAction::Split, 0);
                },
                KeyCode::Char('m') => {
                    return Ok(ModelResponse::NavToMainMenu);
                }
                KeyCode::Char('q') => {
                    return Ok(ModelResponse::Exit);
                }
                _ => {}
            }
        }
        Ok(ModelResponse::Refresh)
    }
}

impl Model for GameScreen {
    fn update(&mut self) -> std::io::Result<ModelResponse> {
        let g_state = (*self.game.get_state()).clone();
        match g_state {
            GameState::WaitingForBet { player_bankroll } => {
                self.handle_waiting_for_bet(player_bankroll)
            },
            GameState::WaitingToDeal {player_bet, player_bankroll} => {
                self.handle_waiting_to_deal(player_bet, player_bankroll)
            },
            GameState::PlayerTurn {..} => {
                self.handle_player_turn()
            }
            GameState::DealerTurn {..} => {
                Ok(ModelResponse::Refresh)
            },
            GameState::RoundComplete {..} => {
                Ok(ModelResponse::Refresh)
            }
        }
    }

    fn ui(&mut self, frame: &mut Frame) {
        let screen = frame.area();
        let screen_layout = Self::create_main_layout(screen);

        let (dealer_horizontal, dealer_vertical) = Self::create_dealer_section(screen_layout[1]);
        self.render_dealer_section(frame, dealer_horizontal, dealer_vertical);

        let (player_horizontal, player_vertical) = Self::create_player_section(screen_layout[2]);
        self.render_player_section(frame, player_horizontal, player_vertical);

        let stats_section = Self::create_stats_section(screen_layout[3]);
        self.render_stats_section(frame, stats_section);

        let footer = Self::create_footer_section(screen_layout[4]);
        render_footer_spans(frame, vec![], footer[1]);
    }
}