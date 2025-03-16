use std::fmt::Display;

fn main() {
    let height: usize = 3;
    let width: usize = 5;

    let mut mine_board: Vec<Vec<bool>> = create_board(height, width, false);
    let mut is_visible_board: Vec<Vec<bool>> = create_board(height, width, true);

    mine_board[0][0] = true;
    mine_board[0][1] = true;
    mine_board[0][3] = true;

    let value_board: Vec<Vec<u8>> = fill_values(&mine_board);
    display_board(&mine_board, &value_board, &is_visible_board);
}

fn display_board(
    mine_board: &Vec<Vec<bool>>,
    value_board: &Vec<Vec<u8>>,
    is_visible_board: &Vec<Vec<bool>>,
) {
    let max_height = mine_board.len();
    let max_width = mine_board[0].len();

    for height in 0..max_height {
        for width in 0..max_width {
            if is_visible_board[height][width] {
                if mine_board[height][width] {
                    print!(" * ")
                } else {
                    print!(" {} ", value_board[height][width])
                }
            } else {
                print!(" ? ")
            }
        }
        print!("\n")
    }
}

fn fill_values(mine_board: &Vec<Vec<bool>>) -> Vec<Vec<u8>> {
    let max_height = mine_board.len();
    let max_width = mine_board[0].len();

    let mut value_board = create_board(mine_board.len(), mine_board[0].len(), 0);
    for (height, row) in mine_board.iter().enumerate() {
        for (width, tile) in row.iter().enumerate() {
            if *tile == true {
                try_increment_nearby_tiles(&mut value_board, max_height, max_width, height, width);
            }
        }
    }

    return value_board;
}

fn try_increment_nearby_tiles(
    value_board: &mut Vec<Vec<u8>>,
    max_height: usize,
    max_width: usize,
    height: usize,
    width: usize,
) {
    let int_height = height as i32;
    let int_width = width as i32;

    for dh in -1..=1 {
        for dw in -1..=1 {
            if does_tile_exist(int_height + dh, int_width + dw, max_height, max_width) {
                value_board[(int_height + dh) as usize][(int_width + dw) as usize] += 1;
            }
        }
    }
}

fn does_tile_exist(height: i32, width: i32, max_height: usize, max_width: usize) -> bool {
    if height < (max_height as i32) && width < (max_width as i32) {
        if height >= 0 && width >= 0 {
            return true;
        }
    }
    return false;
}

fn create_board<T>(height: usize, width: usize, default: T) -> Vec<Vec<T>>
where
    T: Clone,
{
    return vec![vec![default; width]; height];
}

fn debug_print_board<T>(board: &Vec<Vec<T>>)
where
    T: Display,
{
    for row in board {
        for tile in row {
            print!(" {} ", tile)
        }
        print!("\n")
    }
}
