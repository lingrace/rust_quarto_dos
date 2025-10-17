// TODO: input handler game loop
// TODO: handle help commands
use rust_quarto_dos::board::Board;

fn main() {
    println!("organizing code is hard");
    let mut new_board = Board::default();
    println!("{}", new_board);

    let _ = new_board.place_piece(0, 0, 5);
}
