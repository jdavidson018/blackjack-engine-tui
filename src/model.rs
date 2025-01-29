use std::io;
use ratatui::Frame;
use blackjack_engine::game_settings::GameSettings;

#[derive(PartialEq, Debug)]
pub enum ModelResponse {
    /// Check for another update from the screen model
    NoOp,
    /// Run the ui function on the screen model
    Refresh,
    /// Exit the application
    Exit,
    /// Switch to Settings Screen
    NavToSettings,
    NavToHighScores,
    NavToTutorial,
    NavToMainMenu,
    NavToGame {
        game_settings: GameSettings,
    },
    /// QuitGame
    QuitGame,
}

pub trait Model {
    fn update(&mut self) -> io::Result<ModelResponse>;

    /// Called by main program loop to refresh/redraw the current screen
    fn ui(&mut self, frame: &mut Frame);
}

// Note:
// The general idea of this application is simple, we have a loop. That loop
// only knows about one variable, the model. It asks the model to update itself,
// then it asks the model to mutate the given frame.
//
// This allows use to create a loop that functions like a statemachine, where
// the transition logic is separate from the loop logic.
//
// This has the disadvantage of closely tying state to the UI, but that
// can be avoided as Model is only a trait. It could just as easily bet
// implemented using composition, taking objects implementing separate
// UI and an APP traits. Coupling the model and ui works find for my current
// use case.
