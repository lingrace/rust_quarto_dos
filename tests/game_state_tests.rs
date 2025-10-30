use pretty_assertions::{assert_eq, assert_matches};
use rust_quarto_dos::board::{BoardError, Piece};
use rust_quarto_dos::game_state::{GameEngine, GamePhase, GameState, GameStateError, Player};

// TODO: set up shared test setup
// TODO: use loop advancer instead of manually setting up GamePhase for tests
// TODO: turn into a proper test later
#[test]
fn new_game_state_values() {
    let game_state = GameState::new();
    println!("{:?}", game_state);
}

#[test]
fn game_state_current_player() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new();
    game_state.set_player_name(player_1_name, Player::Player1);
    game_state.set_player_name(player_2_name, Player::Player2);
    assert_eq!(game_state.get_current_player(), player_1_name);

    assert!(game_state.select_piece(1).is_ok());
    assert_eq!(game_state.get_current_player(), player_2_name);
}

#[test]
fn game_state_select_piece_success() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new();
    game_state.set_player_name(player_1_name, Player::Player1);
    game_state.set_player_name(player_2_name, Player::Player2);

    let select_res = game_state.select_piece(1);
    assert!(select_res.is_ok());
    assert_matches!(game_state.game_phase, GamePhase::PlacePiece(1));
}

#[test]
fn game_state_select_piece_failure_incorrect_game_phase() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new();
    game_state.set_player_name(player_1_name, Player::Player1);
    game_state.set_player_name(player_2_name, Player::Player2);

    let _ = game_state.select_piece(1);
    let select_res = game_state.select_piece(3);
    assert_matches!(select_res, Err(GameStateError::GamePhaseIncorrect));
}

#[test]
fn game_state_select_piece_failure_invalid_piece() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new();
    game_state.set_player_name(player_1_name, Player::Player1);
    game_state.set_player_name(player_2_name, Player::Player2);

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
    let mut game_state = GameState::new();
    game_state.set_player_name(player_1_name, Player::Player1);
    game_state.set_player_name(player_2_name, Player::Player2);

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

#[test]
fn game_state_place_piece_success() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let piece: i8 = 1;
    let mut game_state = GameState::new();
    game_state.set_player_name(player_1_name, Player::Player1);
    game_state.set_player_name(player_2_name, Player::Player2);

    let _ = game_state.select_piece(piece);
    let place_piece_res = game_state.place_piece(2, 0);
    assert!(place_piece_res.is_ok());
    assert_matches!(game_state.game_phase, GamePhase::SelectPiece);
}

#[test]
fn game_state_place_piece_failure_incorrect_game_phase() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new();
    game_state.set_player_name(player_1_name, Player::Player1);
    game_state.set_player_name(player_2_name, Player::Player2);

    let place_res = game_state.place_piece(1, 1);
    assert_matches!(place_res, Err(GameStateError::GamePhaseIncorrect));
}

#[test]
fn game_state_place_piece_failure_out_of_bounds() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new();
    game_state.set_player_name(player_1_name, Player::Player1);
    game_state.set_player_name(player_2_name, Player::Player2);

    let _ = game_state.select_piece(1);
    let place_res = game_state.place_piece(5, 2);
    assert_matches!(
        place_res,
        Err(GameStateError::PlacePieceError(BoardError::OutOfBounds))
    );
}

#[test]
fn game_state_place_piece_success_is_won() {
    let player_1_name = "player 1";
    let player_2_name = "god";
    let mut game_state = GameState::new();
    game_state.set_player_name(player_1_name, Player::Player1);
    game_state.set_player_name(player_2_name, Player::Player2);

    // Set up win
    let _ = game_state.select_piece(0);
    let _ = game_state.place_piece(0, 0);
    let _ = game_state.select_piece(1);
    let _ = game_state.place_piece(0, 1);
    let _ = game_state.select_piece(2);
    let _ = game_state.place_piece(0, 2);
    let _ = game_state.select_piece(3);
    let win_place_piece_res = game_state.place_piece(0, 3);

    assert!(win_place_piece_res.is_ok());
    assert_matches!(
        game_state.game_phase,
        GamePhase::GameOver(Some(Player::Player1))
    );
}

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
