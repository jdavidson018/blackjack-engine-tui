use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::{error::Error, io};

mod model;
mod app;
mod ui;
mod constants;
mod menu;
mod settings;
mod game;

use crate::menu::menu_screen::MenuScreen;
use crate::model::{Model, ModelResponse};
use crate::settings::settings_screen::SettingsScreen;
use crate::app::App;
use crate::game::game_screen::GameScreen;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}")
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut screen: Box<dyn Model> = Box::new(MenuScreen::new());
    loop {
        terminal.draw(|f| screen.ui(f))?;

        loop {
            // This may look like a nested loop, but it prevents unnecessary
            // re-renders of the terminal ui
            let response = screen.update();
            match response {
                Ok(ModelResponse::Refresh) => break,
                Ok(ModelResponse::Exit) => return Ok(()),
                Ok(ModelResponse::NavToMainMenu) => {
                    screen = Box::new(MenuScreen::new());
                    break;
                }
                Ok(ModelResponse::NavToGame) => {
                    screen = Box::new(GameScreen::new());
                    break;
                }
                Ok(ModelResponse::NavToSettings) => {
                    screen = Box::new(SettingsScreen::new());
                    break;
                }
                Ok(ModelResponse::NavToTutorial) => {
                    screen = Box::new(SettingsScreen::new());
                    break;
                }
                Ok(ModelResponse::NavToHighScores) => {
                    screen = Box::new(SettingsScreen::new());
                    break;
                }
                _ => {}
            }
            break;

        }
    }
}