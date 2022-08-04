use std::io::prelude::*;


#[derive(Copy, Clone, PartialEq)]
enum Spot {
    X,
    O,
    Empty,
}
impl Spot {
    fn to_string(self) -> &'static str {
        match self {
            Spot::X => "X",
            Spot::O => "O",
            Spot::Empty => " ",
        }
    }
    fn next_turn(&mut self) {
        match self {
            Spot::X => *self = Spot::O,
            Spot::O => *self = Spot::X,
            _ => unreachable!(),
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_game(board: [[Spot; 7]; 6], current_turn: Spot) {
    clear_screen();
    for row in board.iter() {
        print!("|");
        for spot in row.iter() {
            print!("{}", spot.to_string());
        }
        println!("|");
    }
    println!("+++++++++");
    println!("+1234567+");
    println!("Current turn: {}", current_turn.to_string());
}

fn invalid_move(board: [[Spot; 7]; 6]) -> usize {
    println!("Invalid move!");
    get_col(board)
}

fn get_col(board: [[Spot; 7]; 6]) -> usize {
    print!("Choose column to place piece: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let char_option = input.trim().chars().next();
    match char_option {
        Some(c) => {
            let num = c.to_digit(10);
            match num {
                Some(n) => {
                    if n > 0 && n < 8 && board[0][(n - 1) as usize] == Spot::Empty {
                        n as usize - 1
                    } else {
                        invalid_move(board)
                    }
                },
                None => invalid_move(board)
            }
        },
        None => invalid_move(board)
    }
}

fn drop_spot(board: &mut [[Spot; 7]; 6], col: usize, current_turn: Spot) {
    for row in (0..6).rev() {
        if board[row][col] == Spot::Empty {
            board[row][col] = current_turn;
            break;
        }
    }
}


fn main() {
    let mut board: [[Spot; 7]; 6] = [[Spot::Empty; 7]; 6];
    let mut current_turn = Spot::X;

    loop {
        print_game(board, current_turn);
        let move_col = get_col(board);
        drop_spot(&mut board, move_col, current_turn);
        current_turn.next_turn();


    }
}
