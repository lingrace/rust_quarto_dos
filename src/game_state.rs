use crate::{
    board::{Board, BoardError, Piece},
    constants::NUM_PIECES,
};

#[derive(Debug)]
pub enum GamePhase {
    SelectPiece,
    PlacePiece(Piece),
    GameOver(Option<Player>),
}

#[derive(Debug, Clone)]
pub enum Player {
    Player1,
    Player2,
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
    pub fn new(player_1_name: &str, player_2_name: &str) -> Self {
        GameState {
            board: Board::default(),
            player_1: player_1_name.to_string(),
            player_2: player_2_name.to_string(),
            current_player: Player::Player1,
            game_phase: GamePhase::SelectPiece,
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
    // TODO: move to GameEngine
    pub fn handle_select_piece(&mut self, input: &str) {
        if let Ok(piece) = input.parse::<Piece>() {
            println!("Selecting piece {}", piece);
            let res = self.select_piece(piece);
            if let Err(e) = res {
                println!("Got an error: {:?}", e);
                println!("These pieces are available: {:?}", self.available_pieces());
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
            let res = self.place_piece(row, col);
            if let Err(e) = res {
                println!("Got an error: {:?}", e);
                println!("Try placing the piece again.");
            }
        }
    }
}
