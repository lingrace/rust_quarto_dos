// take input from user
// handle help commands
// display board
// manage board data (including win conditions)

 
const BOARD_SIZE: usize = 4;


fn main() {
    println!("pet pet");

    let board = [[-1; BOARD_SIZE]; BOARD_SIZE]; // TODO: cannot initialize to -1
    for row in board {
        let mut row_str: String = "".to_string();
        for col in row {
            row_str += col.to_string().as_str();
        }
        println!("{}", row_str);
    }

}
