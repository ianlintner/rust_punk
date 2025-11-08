mod game;
mod items;
mod ui;

use crossterm::event::{self, Event, KeyCode};
use std::io::Result;
use std::time::Duration;

use game::{GameMode, GameState};
use ui::{Renderer, cleanup_terminal, setup_terminal};

fn main() -> Result<()> {
    // Setup terminal
    setup_terminal()?;

    // Create game state
    let width = 60;
    let height = 20;
    let mut game = GameState::new(width, height);

    // Create renderer
    let renderer = Renderer::new(width as u16, height as u16);

    // Game loop
    let result = game_loop(&mut game, &renderer);

    // Cleanup
    cleanup_terminal()?;

    result
}

fn game_loop(game: &mut GameState, renderer: &Renderer) -> Result<()> {
    loop {
        // Render
        renderer.render(game)?;

        // Handle input
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key_event) = event::read()?
        {
            match key_event.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    break;
                }
                _ => {
                    game.handle_input(key_event);
                }
            }
        }

        // Check for game end conditions
        match game.mode {
            GameMode::Victory | GameMode::GameOver => {
                renderer.render(game)?;

                // Wait for Q to quit
                loop {
                    if event::poll(Duration::from_millis(100))?
                        && let Event::Key(key_event) = event::read()?
                        && matches!(key_event.code, KeyCode::Char('q') | KeyCode::Esc)
                    {
                        return Ok(());
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
