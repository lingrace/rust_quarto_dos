use indoc::indoc;
use pretty_assertions::assert_eq;
use rust_quarto_dos::board::{Board, BoardError};

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
