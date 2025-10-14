use rust_quarto_dos::board::Board;

#[test]
fn is_won_horizontal_line() {
    let board = Board::new([
        [0b0000, 0b0010, 0b0100, 0b0001],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
    ]);
    assert!(board.is_won());
}

#[test]
fn is_won_vertical_line() {
    let board = Board::new([
        [0b0000, -1, -1, -1],
        [0b0001, -1, -1, -1],
        [0b0010, -1, -1, -1],
        [0b0100, -1, -1, -1],
    ]);
    assert!(board.is_won());
}

#[test]
fn is_won_diagonal_line() {
    let board = Board::new([
        [0b0000, -1, -1, -1],
        [-1, 0b0001, -1, -1],
        [-1, -1, 0b0010, -1],
        [-1, -1, -1, 0b0100],
    ]);
    assert!(board.is_won());
}

#[test]
fn is_won_counter_diagonal_line() {
    let board = Board::new([
        [-1, -1, -1, 0b0000],
        [-1, -1, 0b0001, -1],
        [-1, 0b0010, -1, -1],
        [0b0100, -1, -1, -1],
    ]);
    assert!(board.is_won());
}

#[test]
fn is_not_won_horizontal_line() {
    let board = Board::new([
        [0b0000, 0b0011, 0b0100, 0b1000],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
        [-1, -1, -1, -1],
    ]);
    assert!(!board.is_won());
}
