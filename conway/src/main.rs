// https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
//
use std::{thread, time};

fn init_board(board: &mut Vec<String>) {
    let n = board.len();
    let m = board[0].len();

    for row in 0..n {
        let row_bytes: &mut [u8] = unsafe { board[row].as_bytes_mut() };
        for col in 0..m {
            row_bytes[col] = '.' as u8;
        }
    }

    for row in 3..6 {
        let row_bytes: &mut [u8] = unsafe { board[row].as_bytes_mut() };
        for col in 3..6 {
            row_bytes[col] = 'x' as u8;
        }
    }
}

fn step(board: &Vec<String>, next_board: &mut Vec<String>) {
    let n = board.len();
    let m = board[0].len();
    for i in 0..n {
        let row_bytes: &mut [u8] = unsafe { next_board[i].as_bytes_mut() };
        for j in 0..m {
            // count live neighbors
            let mut count = 0;
            if board[(i + n - 1) % n].chars().nth((j + m - 1) % m).unwrap() == 'x' {
                count += 1;
            }
            if board[(i + n - 1) % n].chars().nth(j).unwrap() == 'x' {
                count += 1;
            }
            if board[(i + n - 1) % n].chars().nth((j + 1) % m).unwrap() == 'x' {
                count += 1;
            }
            if board[(i + 1) % n].chars().nth((j + m - 1) % m).unwrap() == 'x' {
                count += 1;
            }
            if board[(i + 1) % n].chars().nth(j).unwrap() == 'x' {
                count += 1;
            }
            if board[(i + 1) % n].chars().nth((j + 1) % m).unwrap() == 'x' {
                count += 1;
            }
            if board[i].chars().nth((j + m - 1) % m).unwrap() == 'x' {
                count += 1;
            }
            if board[i].chars().nth((j + 1) % m).unwrap() == 'x' {
                count += 1;
            }

            // init next gen cell as dead
            row_bytes[j] = '.' as u8;

            // rule 1: any live cell with two or three live neighbors survives
            if board[i].chars().nth(j).unwrap() == 'x' && (count == 2 || count == 3) {
                row_bytes[j] = 'x' as u8;
            }

            // rule 2: any dead cell with three live neighbors becomes a live cell
            if board[i].chars().nth(j).unwrap() == '.' && count == 3 {
                row_bytes[j] = 'x' as u8;
            }

            // rule 3: All other live cells die in the next generation. Similarly, all other dead cells stay dead.
            // guaranteed by the next gen cell init
        }
    }
}

fn main() {
    // board n x m (rows x columns)
    let n = 10;
    let m = 10;
    let mut board_a = vec![String::with_capacity(m); n];
    let mut board_b = vec![String::with_capacity(m); n];
    for i in 0..n {
        for _ in 0..m {
            board_a[i].push('.');
            board_b[i].push('.');
        }
    }
    // init board
    init_board(&mut board_a);
    // run game
    for i in 0..10 {
        if i % 2 == 0 {
            step(&board_a, &mut board_b);
            for row in 0..n {
                println!("{}\n", board_b[row]);
            }
        } else {
            step(&board_b, &mut board_a);

            for row in 0..n {
                println!("{}\n", board_a[row]);
            }
        }

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        thread::sleep(time::Duration::from_millis(500));
    }
}
