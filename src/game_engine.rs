use crate::{
    board::Piece,
    game_state::{GameState, Player},
};

pub struct GameEngine {
    pub game_state: GameState, // TODO: consider if and how to make game_state private
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
        }
    }

    pub fn set_player_name(&mut self, input: &str, player: Player) {
        self.game_state.set_player_name(input, player);
    }

    pub fn handle_select_piece(&mut self, input: &str) {
        if let Ok(piece) = input.parse::<Piece>() {
            println!("Selecting piece {}", piece);
            let res = self.game_state.select_piece(piece);
            if let Err(e) = res {
                println!("Got an error: {:?}", e);
                println!(
                    "These pieces are available: {:?}",
                    self.game_state.available_pieces()
                );
                println!("Try selecting a piece again.");
            }
        }
    }

    pub fn handle_place_piece(&mut self, piece: Piece, input: &str) {
        let parts: Vec<&str> = input.split(',').map(|x| x.trim()).collect();
        if parts.len() != 2 {
            println!(
                "Input incorrect, received {} arguments when expected 2",
                parts.len()
            );
            println!("Try placing the piece again.");
            return;
        }
        if let (Ok(row), Ok(col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
            println!("Placing piece {} at row {} and col {}", piece, row, col);
            let res = self.game_state.place_piece(row, col);
            if let Err(e) = res {
                println!("Got an error: {:?}", e);
                println!("Try placing the piece again.");
            }
        }
    }
}
