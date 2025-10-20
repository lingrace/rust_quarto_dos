use std::{collections::HashSet, fmt::Display};

const BOARD_SIZE: usize = 4;
const EMPTY_CELL_VALUE: i8 = -1;
const NUM_PIECES: usize = BOARD_SIZE * BOARD_SIZE;

pub type Piece = i8;
type BoardCells = [[Piece; BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug)]
pub enum BoardError {
    OutOfBounds,
    InvalidPiece,
    PieceAlreadyUsed,
    CellAlreadyUsed,
}

// TODO: move GamePhase and GameState into own module

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

#[derive(Debug)]
pub struct Board {
    cells: BoardCells,
    cell_width: usize, // for Display formatting purposes
}

impl Board {
    pub fn new(cells: BoardCells) -> Self {
        Board {
            cells,
            ..Default::default()
        }
    }

    pub fn is_winning_line(line: &[Piece; BOARD_SIZE]) -> bool {
        let filtered_line: Vec<&Piece> = line.iter().filter(|&&x| x != EMPTY_CELL_VALUE).collect();

        let num_non_empty_cells = filtered_line.len();
        let cumulative_bit_and = filtered_line.iter().fold(0b1111, |acc, x| acc & *x);
        let cumulative_bit_or = filtered_line.iter().fold(0b0000, |acc, x| acc | *x);

        // TODO: remove or turn into a --debug log
        // #[cfg(debug_assertions)]
        // {
        //     println!(
        //         "line: {:?}, num_pieces: {}, cumulative_bit_and: {}, cumulative_bit_or: {}",
        //         line, num_non_empty_cells, cumulative_bit_and, cumulative_bit_or
        //     );
        // }

        return num_non_empty_cells == BOARD_SIZE
            && (cumulative_bit_and != 0b0000 || cumulative_bit_or != 0b1111);
    }

    pub fn available_pieces(&self) -> Vec<Piece> {
        let mut all_available_pieces: HashSet<Piece> = (0..NUM_PIECES as i8).collect();
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.cells[i][j] != EMPTY_CELL_VALUE {
                    all_available_pieces.remove(&self.cells[i][j]);
                }
            }
        }
        let mut all_available_pieces: Vec<Piece> = all_available_pieces.into_iter().collect();
        all_available_pieces.sort();
        all_available_pieces
    }

    pub fn place_piece(&mut self, row: usize, col: usize, piece: Piece) -> Result<(), BoardError> {
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return Err(BoardError::OutOfBounds);
        }

        if piece as usize >= NUM_PIECES || piece < 0 {
            return Err(BoardError::InvalidPiece);
        }

        if self
            .cells
            .into_iter()
            .any(|row| row.into_iter().any(|cell| cell == piece))
        {
            return Err(BoardError::PieceAlreadyUsed);
        }

        if self.cells[row][col] != EMPTY_CELL_VALUE {
            return Err(BoardError::CellAlreadyUsed);
        }

        self.cells[row][col] = piece;

        Ok(())
    }

    pub fn is_won(&self) -> bool {
        // Check rows
        for row in self.cells {
            if Board::is_winning_line(&row) {
                return true;
            }
        }

        // Check columns
        for j in 0..BOARD_SIZE {
            let mut col = [EMPTY_CELL_VALUE; BOARD_SIZE];
            for i in 0..BOARD_SIZE {
                col[i] = self.cells[i][j];
            }
            if Board::is_winning_line(&col) {
                return true;
            }
        }

        // Check diagonals
        let mut diag = [EMPTY_CELL_VALUE; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            diag[i] = self.cells[i][i];
        }
        if Board::is_winning_line(&diag) {
            return true;
        }

        let mut cross_diag = [EMPTY_CELL_VALUE; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            cross_diag[i] = self.cells[i][BOARD_SIZE - 1 - i];
        }
        if Board::is_winning_line(&cross_diag) {
            return true;
        }

        false
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            cells: [[EMPTY_CELL_VALUE; BOARD_SIZE]; BOARD_SIZE],
            cell_width: 4,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = "─".repeat(self.cell_width * BOARD_SIZE + 3 * (BOARD_SIZE - 1) + 4);
        let board_str = self
            .cells
            .iter()
            .map(|row| {
                let row_str = row
                    .iter()
                    .map(|x| {
                        if *x != EMPTY_CELL_VALUE {
                            format!("{:width$}", x, width = self.cell_width)
                        } else {
                            format!("{:width$}", "", width = self.cell_width)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" │ ");
                format!("│ {} │", row_str)
            })
            .collect::<Vec<_>>()
            .join(&format!("\n{}\n", sep));

        return write!(f, "{}\n{}\n{}", sep, board_str, sep);
    }
}
