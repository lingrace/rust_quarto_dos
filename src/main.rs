use std::io;

use rust_quarto_dos::{
    constants::{INSTRUCTIONS, NUM_PLAYERS},
    game_state::{GameEngine, GamePhase},
};

// TODO: add tests
fn query_for_player_names() -> [String; NUM_PLAYERS] {
    let mut player_array: [String; NUM_PLAYERS] = ["".to_string(), "".to_string()];
    for i in 0..NUM_PLAYERS {
        println!("Please enter a name for Player {}: ", i + 1);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        // TODO: add input validation
        player_array[i] = input.to_string();
    }
    player_array
}

fn main() {
    // TODO: Refactor this so that the init steps are also part of the input loop
    println!("Welcome to Quarto");
    println!("{}", INSTRUCTIONS);
    let [player_1_name, player_2_name] = query_for_player_names();
    let mut game_engine = GameEngine::new(&player_1_name, &player_2_name);

    loop {
        let mut input = String::new();

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
        match game_engine.game_state.game_phase {
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
    }
}
