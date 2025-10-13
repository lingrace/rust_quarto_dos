// take input from user
// handle help commands
// display board
// manage board data (including win conditions)

use rust_quarto_dos::board::Board;

fn main() {
    println!("you can win now");
    let mut new_board = Board::default();
    println!("{}", new_board);

    let res = new_board.place_piece(0, 0, 5);
}
