use pretty_assertions::assert_eq;
use rust_quarto_dos::board::{Board, BoardError, is_winning_line};

#[test]
fn is_winning_line_true() {
    assert!(is_winning_line([0, 1, 2, 3]));
}

#[test]
fn is_winning_line_false() {
    assert!(!is_winning_line([0, 1, 2, 14]));
}
