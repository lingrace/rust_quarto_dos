use std::io;

use rust_quarto_dos::game_state::{GameEngine, GamePhase};

fn main() {
    println!("choo choo");
    let mut game_engine = GameEngine::new("p1", "god");

    loop {
        let mut input = String::new();
        println!("Enter something (or 'quit' to exit):");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim(); // remove newline and spaces

        match input {
            // TODO: handle help commands
            "display board" => {
                game_engine.game_state.display_board();
            }
            _ => match game_engine.game_state.game_phase {
                // TODO: improve input prompting
                GamePhase::SelectPiece => game_engine.handle_select_piece(input),
                GamePhase::PlacePiece(piece) => {
                    game_engine.handle_place_piece(piece, input);
                }
                GamePhase::GameOver(_) => {
                    println!("game is over!");
                }
            },
        }

        // TODO: after processing input, perhaps add a match statement to print the prompt for the next input
    }
}
