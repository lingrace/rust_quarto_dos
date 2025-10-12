// take input from user
// handle help commands
// display board
// manage board data (including win conditions)

 
const BOARD_SIZE: usize = 4;


struct Board {
    cells: [[i32; BOARD_SIZE]; BOARD_SIZE]
}

impl Board {
    fn new() -> Self {
        Board {
            cells:  [[-1; BOARD_SIZE]; BOARD_SIZE]
        }
    }
}

fn print_board(board: &Board) {
    for row in board.cells {
        let mut row_str: String = "".to_string();
        for col in row {
            row_str += col.to_string().as_str();
        }
        println!("{}", row_str);
    }
}

fn main() {
    println!("aiya");
    let board = Board::new();
    print_board(&board);
}