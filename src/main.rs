// take input from user
// handle help commands
// display board
// manage board data (including win conditions)

 
const BOARD_SIZE: usize = 4;

type Board = [[i32; BOARD_SIZE]; BOARD_SIZE];

fn print_board(board: &Board) {
    for row in board {
        let mut row_str: String = "".to_string();
        for col in row {
            row_str += col.to_string().as_str();
        }
        println!("{}", row_str);
    }
}

fn main() {
    println!("and board");
    let board: Board = [[-1; BOARD_SIZE]; BOARD_SIZE]; // TODO: should not initialize to -1
    print_board(&board);
    print_board(&board);
}
