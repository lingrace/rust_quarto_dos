use std::io;

use crate::{constants::INSTRUCTIONS, game_engine::GameEngine, game_state::GamePhase};

pub trait GameIO {
    fn input(&self) -> String;
    fn output(&self, str: String);
}

pub struct ConsoleGameIO {}
impl GameIO for ConsoleGameIO {
    fn input(&self) -> String {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input
    }

    fn output(&self, string: String) {
        println!("{}", string);
    }
}

// TODO: test game loop
pub fn game_loop(io: Box<dyn GameIO>) {
    let mut game_engine = GameEngine::new();

    io.output(format!("{}", INSTRUCTIONS));

    // TODO: will eventually go in game engine
    loop {
        // Output instructions
        match game_engine.game_state.game_phase {
            GamePhase::SetNameForPlayer(player) => {
                io.output(format!("Enter a name for {}.", player));
            }
            GamePhase::SelectPiece => {
                io.output(format!(
                    "{}, it's time to select a piece!",
                    game_engine.game_state.get_current_player()
                ));
                io.output(format!(
                    "available pieces: {:?}",
                    game_engine.game_state.available_pieces()
                ));
                io.output(format!("Select a piece by id, e.g. 3, then enter."))
            }
            GamePhase::PlacePiece(piece) => {
                io.output(format!(
                    "{}, it's time to place piece {}!",
                    game_engine.game_state.get_current_player(),
                    piece
                ));
                io.output(format!(
                    "Place piece by specifying row and column, e.g. 1, 2, then enter."
                ));
            }
            GamePhase::GameOver(option_player) => {
                io.output(format!("game is over!"));
                match option_player {
                    Some(_) => {
                        io.output(format!(
                            "{} has won! Congratulations!",
                            game_engine.game_state.get_current_player()
                        ));
                    }
                    None => {
                        io.output(format!("This game ended in a stalemate."));
                    }
                }
            }
        }

        let input = io.input(); // remove newline and spaces
        let input = input.trim();

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
