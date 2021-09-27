// https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
//
use std::{thread, time};

fn init_board(board: &mut Vec<Vec<char>>) {
    let n = board.len();
    let m = board[0].len();

    for row in 0..n {
        for col in 0..m {
            board[row][col] = '.';
        }
    }

    for row in 3..6 {
        for col in 2..7 {
            board[row][col] = 'x';
        }
    }
}

fn is_alive(c: char) -> bool {
    return c == 'x' || c == 'l';
}

fn step(board: &mut Vec<Vec<char>>) {
    let n = board.len();
    let m = board[0].len();
    // first pass to compute new state (with special characters)
    for row in 0..n {
        for col in 0..m {
            // count live neighbors
            let mut count = 0;
            for i in 0..3 {
                for j in 0..3 {
                    if i == 1 && j == 1 {
                        continue;
                    }
                    let r = row as i32 + i - 1;
                    let c = col as i32 + j - 1;

                    if 0 <= r && r < n as i32 && 0 <= c && c < m as i32 {
                        if is_alive(board[r as usize][c as usize]) {
                            //if is_alive(board[(row + n + i - 1) % n][(col + m + j - 1) % m]) {
                            count += 1;
                        }
                    }
                }
            }
            // current cell value
            let c = board[row][col];

            if c == 'x' && (count == 2 || count == 3) {
                // rule 1: any live cell with two or three live neighbors survives
                board[row][col] = 'x';
            } else if c == '.' && count == 3 {
                // rule 2: any dead cell with three live neighbors becomes a live cell
                // lets mark with 'd' cells that were dead and are now alive
                board[row][col] = 'd';
            }
            // rule 3: All other live cells die in the next generation. Similarly, all other dead cells stay dead.
            else if c == 'x' {
                // lets mark with 'l' cells that were live and are now dead
                board[row][col] = 'l';
            } else {
                board[row][col] = '.';
            }
        }
    }
    // second pass to replace special characters
    for row in 0..n {
        for col in 0..m {
            if board[row][col] == 'l' {
                board[row][col] = '.';
            } else if board[row][col] == 'd' {
                board[row][col] = 'x';
            }
        }
    }
}

fn main() {
    // board n x m (rows x columns)
    let n = 10;
    let m = 10;
    let row = vec!['.'; m];
    let mut board = vec![row; n];

    // init board
    init_board(&mut board);

    // run game
    for _ in 0..10 {
        step(&mut board);

        for row in 0..n {
            for col in 0..m {
                print!("{}", board[row][col]);
            }
            println!("");
        }

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        thread::sleep(time::Duration::from_millis(500));
    }
}
