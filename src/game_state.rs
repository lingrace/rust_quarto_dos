use std::fmt::Display;

use crate::{
    board::{Board, BoardError, Piece},
    constants::NUM_PIECES,
};

// TODO: Merge SetNameForPlayer into 1
#[derive(Debug, PartialEq)]
pub enum GamePhase {
    SetNameForPlayer(Player),
    SelectPiece,
    PlacePiece(Piece),
    GameOver(Option<Player>),
}
// TODO you could just add more options for the rename phases

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::Player1 => {
                write!(f, "Player 1")
            }
            Player::Player2 => {
                write!(f, "Player 2")
            }
        }
    }
}

#[derive(Debug)]
pub enum GameStateError {
    SelectPieceError(BoardError),
    PlacePieceError(BoardError),
    GamePhaseIncorrect,
}

#[derive(Debug)]
pub struct GameState {
    board: Board,
    player_1: String,
    player_2: String,
    current_player: Player,
    pub game_phase: GamePhase,
}

impl GameState {
    // TODO: write a function that lets you create a GameState initialized with arbitraty state
    pub fn new() -> Self {
        GameState {
            board: Board::default(),
            player_1: "Player 1".to_string(),
            player_2: "Player 2".to_string(),
            current_player: Player::Player1,
            game_phase: GamePhase::SetNameForPlayer(Player::Player1),
        }
    }

    pub fn set_player_name(&mut self, player_name: &str, player: Player) {
        // TODO: add str validation
        // TODO: if str empty, no op

        match player {
            Player::Player1 => {
                self.player_1 = player_name.to_string();

                if self.game_phase == GamePhase::SetNameForPlayer(Player::Player1) {
                    self.game_phase = GamePhase::SetNameForPlayer(Player::Player2)
                }
            }
            Player::Player2 => {
                self.player_2 = player_name.to_string();
                if self.game_phase == GamePhase::SetNameForPlayer(Player::Player2) {
                    self.game_phase = GamePhase::SelectPiece
                }
            }
        }
    }

    pub fn set_player_2_name(&mut self, player_2_name: &str) {
        // TODO: add str validation
        // TODO: if str empty, no op
        self.player_2 = player_2_name.to_string();
        if self.game_phase == GamePhase::SetNameForPlayer(Player::Player2) {
            self.game_phase = GamePhase::SelectPiece
        }
    }

    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }

    pub fn available_pieces(&self) -> Vec<Piece> {
        self.board.available_pieces()
    }

    pub fn display_board(&self) {
        println!("{}", self.board);
    }

    pub fn get_current_player(&self) -> &str {
        match self.current_player {
            Player::Player1 => self.player_1.as_str(),
            Player::Player2 => self.player_2.as_str(),
        }
    }

    pub fn select_piece(&mut self, piece: Piece) -> Result<(), GameStateError> {
        if !matches!(self.game_phase, GamePhase::SelectPiece) {
            return Err(GameStateError::GamePhaseIncorrect);
        }

        if piece as usize >= NUM_PIECES {
            return Err(GameStateError::SelectPieceError(BoardError::InvalidPiece));
        }

        if !self.board.available_pieces().contains(&piece) {
            return Err(GameStateError::SelectPieceError(
                BoardError::PieceAlreadyUsed,
            ));
        }

        self.switch_player();
        self.game_phase = GamePhase::PlacePiece(piece);

        Ok(())
    }

    pub fn place_piece(&mut self, row: usize, col: usize) -> Result<(), GameStateError> {
        let piece = match self.game_phase {
            GamePhase::PlacePiece(piece) => Some(piece),
            _ => None,
        };
        if piece.is_none() {
            return Err(GameStateError::GamePhaseIncorrect);
        }

        let piece = piece.unwrap();

        match self.board.place_piece(row, col, piece) {
            Err(err) => return Err(GameStateError::PlacePieceError(err)),
            _ => {}
        }

        self.game_phase = if self.board.is_won() {
            GamePhase::GameOver(Some(self.current_player.clone()))
        } else if self.board.available_pieces().is_empty() {
            GamePhase::GameOver(None)
        } else {
            GamePhase::SelectPiece
        };
        Ok(())
    }
}

// TODO: break GameEngine into a new file
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
