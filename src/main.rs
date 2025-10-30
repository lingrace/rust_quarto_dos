use std::io;

use rust_quarto_dos::{
    constants::INSTRUCTIONS,
    game_state::{GameEngine, GamePhase},
};

fn main() {
    let mut game_engine = GameEngine::new();

    println!("{}", INSTRUCTIONS);

    // TODO: will eventually go in game engine
    loop {
        // Output instructions
        match game_engine.game_state.game_phase {
            GamePhase::SetNameForPlayer(player) => {
                println!("Enter a name for {}.", player);
            }
            GamePhase::SelectPiece => {
                println!(
                    "{}, it's time to select a piece!",
                    game_engine.game_state.get_current_player()
                );
                println!(
                    "available pieces: {:?}",
                    game_engine.game_state.available_pieces()
                );
                println!("Select a piece by id, e.g. 3, then enter.")
            }
            GamePhase::PlacePiece(piece) => {
                println!(
                    "{}, it's time to place piece {}!",
                    game_engine.game_state.get_current_player(),
                    piece
                );
                println!("Place piece by specifying row and colum, e.g. 1, 2, then enter.");
            }
            GamePhase::GameOver(option_player) => {
                println!("game is over!");
                match option_player {
                    Some(_) => {
                        println!(
                            "{} has won! Congratulations!",
                            game_engine.game_state.get_current_player()
                        );
                    }
                    None => {
                        println!("This game ended in a stalemate.");
                    }
                }
            }
        }

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim(); // remove newline and spaces

        // Handle input
        match input {
            // TODO: handle help commands
            "display board" => {
                game_engine.game_state.display_board();
            }
            "rename player {} {}" => {} // TODO: handle renaming commands
            _ => match game_engine.game_state.game_phase {
                // TODO: improve input prompting
                GamePhase::SetNameForPlayer(player) => {
                    game_engine.set_player_name(input, player);
                }
                GamePhase::SelectPiece => game_engine.handle_select_piece(input),
                GamePhase::PlacePiece(piece) => {
                    game_engine.handle_place_piece(piece, input);
                }
                GamePhase::GameOver(_) => {}
            },
        }
    }
}
