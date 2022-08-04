
#[derive(Copy, Clone)]
enum Spot {
    X,
    O,
    Empty,
}
impl Spot {
    fn to_string(&self) -> &'static str {
        match self {
            Spot::X => "X",
            Spot::O => "O",
            Spot::Empty => " ",
        }
    }
}

fn print_board(board: [[Spot; 7]; 6]) {
    for row in board.iter() {
        print!("|");
        for spot in row.iter() {
            print!("{}", spot.to_string());
        }
        println!("|");
    }
    println!("+++++++++");
    println!("+1234567+");
}


fn main() {
    let mut board: [[Spot; 7]; 6] = [[Spot::Empty; 7]; 6];
    print_board(board);
}
