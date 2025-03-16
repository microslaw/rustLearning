fn main() {
    let mut board: [[i8; 3]; 3] = [[0; 3]; 3];
    board[0][0] = 1;
    board[0][1] = 2;
    print_board(board)
}

fn create_board(height: i8, width: i8) {
    return vect![vec!['#'; 80]; 24];
}

fn print_board(board: [[i8; 3]; 3]) {
    for row in board {
        for tile in row {
            print!(" {} ", tile)
        }
        print!("\n")
    }
}
