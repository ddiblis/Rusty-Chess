// Define an enum to represent the color of pieces
#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White,
}

// Define an enum to represent chess pieces
#[derive(Debug, Clone, Copy)]
enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
    Empty,
}

impl Piece {
    // Function to convert the piece enum variant to its Unicode representation
    fn to_unicode(&self) -> char {
        match *self {
            Piece::King(Color::White) => '♔',
            Piece::Queen(Color::White) => '♕',
            Piece::Rook(Color::White) => '♖',
            Piece::Bishop(Color::White) => '♗',
            Piece::Knight(Color::White) => '♘',
            Piece::Pawn(Color::White) => '♙',
            Piece::King(Color::Black) => '♚',
            Piece::Queen(Color::Black) => '♛',
            Piece::Rook(Color::Black) => '♜',
            Piece::Bishop(Color::Black) => '♝',
            Piece::Knight(Color::Black) => '♞',
            Piece::Pawn(Color::Black) => '♟',
            Piece::Empty => ' ',
        }
    }
}

// Define the size of the chessboard as a const
const SIZE: usize = 8;

fn main() {
    // Create a 2D array to represent the chessboard
    let mut board = [[Piece::Empty; SIZE]; SIZE];

    // Initialize the board
    initialize_board(&mut board);

    // Print the labeled chessboard
    print_board(&board);
}

// Function to initialize the chessboard with pieces
fn initialize_board(board: &mut [[Piece; SIZE]; SIZE]) {
    // Initialize the starting positions of some pieces for white
    board[0][0] = Piece::Rook(Color::White);
    board[0][1] = Piece::Knight(Color::White);
    board[0][2] = Piece::Bishop(Color::White);
    board[0][3] = Piece::Queen(Color::White);
    board[0][4] = Piece::King(Color::White);
    board[0][5] = Piece::Bishop(Color::White);
    board[0][6] = Piece::Knight(Color::White);
    board[0][7] = Piece::Rook(Color::White);
    for i in 0..SIZE {
        board[1][i] = Piece::Pawn(Color::White);
        board[SIZE - 2][i] = Piece::Pawn(Color::Black);
    }
    // Initialize the starting positions of some pieces for black
    board[SIZE - 1][0] = Piece::Rook(Color::Black);
    board[SIZE - 1][1] = Piece::Knight(Color::Black);
    board[SIZE - 1][2] = Piece::Bishop(Color::Black);
    board[SIZE - 1][3] = Piece::Queen(Color::Black);
    board[SIZE - 1][4] = Piece::King(Color::Black);
    board[SIZE - 1][5] = Piece::Bishop(Color::Black);
    board[SIZE - 1][6] = Piece::Knight(Color::Black);
    board[SIZE - 1][7] = Piece::Rook(Color::Black);
}

// Function to print the chessboard
fn print_board(board: &[[Piece; SIZE]; SIZE]) {
    // Print column labels
    println!("    A   B   C   D   E   F   G   H");
    // Print top border
    println!("  +---+---+---+---+---+---+---+---+");

    // Iterate over each row
    for (i, row) in board.iter().enumerate() {
        // Print the row number
        print!("{} ", 8 - i);

        // Print squares in the row
        for (j, square) in row.iter().enumerate() {
            // Print the piece
            match *square {
                Piece::Empty => {
                    let is_black = (i + j) % 2 == 0;
                    if is_black {
                        print!("| {} ", '■'); // Unicode black square
                    } else {
                        print!("| {} ", '□'); // Unicode white square
                    }
                }
                _ => print!("| {} ", square.to_unicode()),
            }
        }

        // Print right border
        println!("|");

        // Print horizontal grid line or bottom border
        println!("  +---+---+---+---+---+---+---+---+");
    }
}