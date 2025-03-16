// use std::fmt::Display;
// use std::io::{self, Write};
use std::{fmt::Debug, io, str::FromStr};

enum GameStatus {
    Won,
    Lost,
    Continues,
}

fn main() {
    let height: usize = 4;
    let width: usize = 5;

    let mut mine_board: Vec<Vec<bool>> = create_board(height, width, false);
    let mut is_visible_board: Vec<Vec<bool>> = create_board(height, width, false);

    mine_board[0][2] = true;
    mine_board[1][2] = true;
    mine_board[2][2] = true;
    mine_board[3][2] = true;
    let value_board: Vec<Vec<u8>> = fill_values(&mine_board);
    let visible_board: Vec<Vec<bool>> = create_board(height, width, true);
    let mut game_status: GameStatus;

    loop {
        display_board(&mine_board, &value_board, &is_visible_board);
        make_turn(&mut is_visible_board, &value_board, height, width);

        game_status = check_game_status(&is_visible_board, &mine_board);

        match game_status {
            GameStatus::Continues => continue,
            GameStatus::Lost => {
                println!("Game lost");
                display_board(&mine_board, &value_board, &visible_board);
                break;
            }
            GameStatus::Won => {
                println!("Game won");
                display_board(&mine_board, &value_board, &visible_board);
                break;
            }
        }
    }
}

// 4 Different situations per tile are possible:
// visible and mine => game is lost
// visible and not mine => game continues, unless all non-mines are, then game won
// not visible and mine => game continues, unless all mines and only mines are, then game won
// not visible and not mine => game continues, if at least one exists

fn check_game_status(is_visible_board: &Vec<Vec<bool>>, mine_board: &Vec<Vec<bool>>) -> GameStatus {
    let mut any_left_to_discover: bool = false;
    for (row_visible, row_mine) in is_visible_board.iter().zip(mine_board.iter()) {
        for (is_visible, is_mine) in row_visible.iter().zip(row_mine.iter()) {
            if *is_visible && *is_mine {
                return GameStatus::Lost;
            }
            if !*is_visible && !*is_mine {
                any_left_to_discover = true;
            }
        }
    }

    if !any_left_to_discover {
        GameStatus::Won
    } else {
        GameStatus::Continues
    }
}

fn make_turn(
    is_visible_board: &mut Vec<Vec<bool>>,
    value_board: &Vec<Vec<u8>>,
    height: usize,
    width: usize,
) {
    let mut picked_height: usize;
    let mut picked_width: usize;

    loop {
        picked_height = get_input("What height would you like to check? \n");
        picked_width = get_input("What width would you like to check? \n");

        if does_tile_exist(picked_height as i32, picked_width as i32, height, width) {
            if !is_visible_board[picked_height][picked_width] {
                break;
            }
        }
        println!(
            "Tile {} {} is invalid; Please pick different tile",
            picked_height, picked_width
        )
    }

    floodfill_visible(value_board, is_visible_board, picked_height, picked_width);
}

fn floodfill_visible(
    value_board: &Vec<Vec<u8>>,
    is_visible_board: &mut Vec<Vec<bool>>,
    start_height: usize,
    start_width: usize,
) {
    let mut queue: Vec<(usize, usize)> = vec![(start_height, start_width)];

    let max_height = is_visible_board.len();
    let max_width = is_visible_board[0].len();

    while !queue.is_empty() {
        let tuple: Option<(usize, usize)> = queue.pop();
        let (height, width) = match tuple {
            Some(x) => x,
            None => (0, 0),
        };

        // if the examined tile is already revealed, continue
        if is_visible_board[height][width] == true {
            continue;
        }

        is_visible_board[height][width] = true;
        // if currently examined tile is not neighbouring with a mine, add it's neighbours for examination
        if value_board[height][width] == 0 {
            for dh in -1..=1 {
                for dw in -1..=1 {
                    let bordering_height: i32 = height as i32 + dh;
                    let bordering_width: i32 = width as i32 + dw;

                    if does_tile_exist(bordering_height, bordering_width, max_height, max_width) {
                        queue.push((bordering_height as usize, bordering_width as usize));
                    }
                }
            }
        }
    }
}

fn get_input<T>(message: &str) -> T
where
    T: Debug,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    print!(" {} ", message);
    let mut x = String::with_capacity(16);
    io::stdin().read_line(&mut x).expect("Error reading input");
    let x: T = x.trim().parse().expect("Error parsing number");
    return x;
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
