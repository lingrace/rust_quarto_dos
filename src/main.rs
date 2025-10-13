// take input from user
// handle help commands
// display board
// manage board data (including win conditions)

use indoc::indoc; // used in tests
use pretty_assertions::assert_eq; // used in tests
use std::fmt::Display;

const BOARD_SIZE: usize = 4;
const EMPTY_CELL_VALUE: i8 = -1;
const NUM_PIECES: usize = BOARD_SIZE * BOARD_SIZE;

type Piece = i8;
type BoardCells = [[Piece; BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug)]
enum BoardError {
    OutOfBounds,
    InvalidPiece,
    PieceAlreadyUsed,
    CellAlreadyUsed,
}

struct Board {
    cells: BoardCells,
    cell_width: usize,
}

impl Board {
    fn new(cells: BoardCells) -> Self {
        Board {
            cells,
            ..Default::default()
        }
    }

    fn place_piece(&mut self, row: usize, col: usize, piece: Piece) -> Result<(), BoardError> {
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
        let sep = "─".repeat(self.cell_width * BOARD_SIZE + 3 * (BOARD_SIZE - 1) + 2);
        let board_str = self
            .cells
            .iter()
            .map(|row| {
                let row_str = row
                    .iter()
                    .map(|x| format!("{:width$}", x, width = self.cell_width))
                    .collect::<Vec<_>>()
                    .join(" │ ");
                format!("│{}│", row_str)
            })
            .collect::<Vec<_>>()
            .join(&format!("\n{}\n", sep));
        // TODO add line separator

        return write!(f, "{}\n{}\n{}", sep, board_str, sep);
    }
}
fn main() {
    println!("ahahhhhhhh");
    let mut new_board = Board::default();
    println!("{}", new_board);

    let res = new_board.place_piece(0, 0, 5);
}

#[test]
fn place_piece_success() {
    let mut new_board = Board::default();
    let place_piece_result = new_board.place_piece(0, 0, 5);
    assert!(place_piece_result.is_ok());
    let expected_board_str = indoc! {
        "
            ───────────────────────────
            │   5 │   -1 │   -1 │   -1│
            ───────────────────────────
            │  -1 │   -1 │   -1 │   -1│
            ───────────────────────────
            │  -1 │   -1 │   -1 │   -1│
            ───────────────────────────
            │  -1 │   -1 │   -1 │   -1│
            ───────────────────────────"
    };
    assert_eq!(format!("{}", new_board), expected_board_str);
}

#[test]
fn place_same_piece_failure() {
    let mut new_board = Board::default();
    let _ = new_board.place_piece(0, 0, 5);
    let place_same_piece_result = new_board.place_piece(0, 1, 5);
    assert!(matches!(
        place_same_piece_result,
        Err(BoardError::PieceAlreadyUsed)
    ));
}

#[test]
fn place_piece_in_occupied_territory_failure() {
    let mut new_board = Board::default();
    let _ = new_board.place_piece(0, 0, 5);
    let place_piece_in_same_cell_result = new_board.place_piece(0, 0, 7);
    assert!(matches!(
        place_piece_in_same_cell_result,
        Err(BoardError::CellAlreadyUsed)
    ));
}

#[test]
fn place_piece_out_of_bounds_failure() {
    let mut new_board = Board::default();
    let place_piece_result = new_board.place_piece(0, 7, 5);
    assert!(matches!(place_piece_result, Err(BoardError::OutOfBounds)));
}

#[test]
fn place_invalid_piece_failure() {
    let mut new_board = Board::default();
    let place_piece_result = new_board.place_piece(0, 0, 25);
    assert!(matches!(place_piece_result, Err(BoardError::InvalidPiece)));
}
