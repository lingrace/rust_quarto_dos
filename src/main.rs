// TODO: replace all printlns in game_state or board into IO

use rust_quarto_dos::game_loop::{self, ConsoleGameIO};

fn main() {
    println!("refactor GameLoop into own file");
    game_loop::game_loop(Box::new(ConsoleGameIO {}));
}
