use pretty_assertions::{assert_eq, assert_matches};
use rust_quarto_dos::board::{BoardError, GameState, GameStateError};

// TODO turn into a proper test later
#[test]
fn new_game_state_values() {
    let game_state = GameState::new("player 1", "god");
    println!("{:?}", game_state);
}

#[test]
fn game_state_current_player() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new(player_1_name, player_2_name);
    assert_eq!(game_state.current_player(), player_1_name);
    assert!(game_state.select_piece(1).is_ok());
    assert_eq!(game_state.current_player(), player_2_name);
}

#[test]
fn game_state_select_piece_success() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new(player_1_name, player_2_name);
    let select_res = game_state.select_piece(1);
    assert!(select_res.is_ok());
}

#[test]
fn game_state_select_piece_failure_incorrect_game_phase() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new(player_1_name, player_2_name);
    let _ = game_state.select_piece(1);
    let select_res = game_state.select_piece(3);
    assert_matches!(select_res, Err(GameStateError::GamePhaseIncorrect));
}

#[test]
fn game_state_select_piece_failure_invalid_piece() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new(player_1_name, player_2_name);
    let select_res = game_state.select_piece(111);
    assert_matches!(
        select_res,
        Err(GameStateError::SelectPieceError(BoardError::InvalidPiece))
    );
}

#[test]
fn game_state_select_piece_failure_piece_already_used() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let piece: i8 = 1;
    let mut game_state = GameState::new(player_1_name, player_2_name);
    let select_res = game_state.select_piece(piece);
    assert!(select_res.is_ok());
    let place_res = game_state.place_piece(1, 1);
    assert!(place_res.is_ok());
    let select_res = game_state.select_piece(piece);
    assert_matches!(
        select_res,
        Err(GameStateError::SelectPieceError(
            BoardError::PieceAlreadyUsed
        ))
    );
}
