use rust_quarto_dos::board::GameState;

// TODO turn into a proper test later
#[test]
fn new_game_state_values() {
    let game_state = GameState::new("player 1".to_string(), "god".to_string());
    println!("{:?}", game_state);
}
