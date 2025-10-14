use pretty_assertions::assert_eq;
use std::collections::HashSet;

use rust_quarto_dos::board::{Board, Piece};

#[test]
fn available_pieces_success() {
    let board = Board::new([
        [0b0000, -1, -1, -1],
        [-1, 0b0010, -1, -1],
        [-1, 0b0100, -1, -1],
        [-1, -1, 0b0001, -1],
    ]);
    let expected_pieces: HashSet<Piece> = [3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        .into_iter()
        .collect();

    println!("{:?}", board.available_pieces());
    assert_eq!(board.available_pieces(), expected_pieces);
}
