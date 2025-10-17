use rust_quarto_dos::board::Board;

#[test]
fn is_not_winning_line_insufficient_pieces() {
    assert!(!Board::is_winning_line(&([0, 1, -1, -1])));
}

#[test]
fn is_not_winning_line_pieces_do_not_share_trait() {
    assert!(!Board::is_winning_line(&([0b0001, 0b0010, 0b0100, 0b1000])));
}

#[test]
fn is_winning_line_for_pieces_that_share_0() {
    assert!(Board::is_winning_line(&([0b0000, 0b0001, 0b0010, 0b0100])));
}

#[test]
fn is_winning_line_for_pieces_that_share_1() {
    assert!(Board::is_winning_line(&([0b1000, 0b1001, 0b1010, 0b1100])));
}
