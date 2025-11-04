// TODO: replace all printlns in game_state or board into IO

use rust_quarto_dos::game_loop::{self, ConsoleGameIO};

fn main() {
    println!("testing game loop ahhhh");
    game_loop::game_loop(&mut ConsoleGameIO {});
}
