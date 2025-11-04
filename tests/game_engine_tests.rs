use pretty_assertions::assert_matches;

use rust_quarto_dos::{
    board::Piece,
    game_engine::GameEngine,
    game_state::{GamePhase, Player},
};

#[test]
fn game_state_handle_select_piece_success() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_engine = GameEngine::new();
    game_engine
        .game_state
        .set_player_name(player_1_name, Player::Player1);
    game_engine
        .game_state
        .set_player_name(player_2_name, Player::Player2);

    game_engine.handle_select_piece("1");
    let _expected_available_pieces: Vec<Piece> =
        vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    assert_matches!(game_engine.game_state.game_phase, GamePhase::PlacePiece(1));
    assert_matches!(
        game_engine.game_state.available_pieces(),
        _expected_available_pieces
    );
}

#[test]
fn game_state_handle_select_piece_failure() {
    let player_1_name = "player 1";
    let player_2_name = "god";

    let mut game_engine = GameEngine::new();
    game_engine
        .game_state
        .set_player_name(player_1_name, Player::Player1);
    game_engine
        .game_state
        .set_player_name(player_2_name, Player::Player2);

    game_engine.handle_select_piece("100");
    assert_matches!(game_engine.game_state.game_phase, GamePhase::SelectPiece);
}

#[test]
fn game_state_handle_place_piece_success() {
    let player_1_name = "player 1";
    let player_2_name = "god";

    let mut game_engine = GameEngine::new();
    game_engine
        .game_state
        .set_player_name(player_1_name, Player::Player1);
    game_engine
        .game_state
        .set_player_name(player_2_name, Player::Player2);

    // setup
    game_engine.handle_select_piece("1");
    let _expected_available_pieces: Vec<Piece> =
        vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // test
    game_engine.handle_place_piece(1, "1, 1");
    assert_matches!(game_engine.game_state.game_phase, GamePhase::SelectPiece);
}

#[test]
fn game_state_handle_place_piece_failure() {
    let player_1_name = "player 1";
    let player_2_name = "god";

    let mut game_engine = GameEngine::new();
    game_engine
        .game_state
        .set_player_name(player_1_name, Player::Player1);
    game_engine
        .game_state
        .set_player_name(player_2_name, Player::Player2);

    // setup
    game_engine.handle_select_piece("1");
    let _expected_available_pieces: Vec<Piece> =
        vec![0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // test
    game_engine.handle_place_piece(1, "1 1");
    assert_matches!(game_engine.game_state.game_phase, GamePhase::PlacePiece(1));
}
