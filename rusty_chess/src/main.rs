pub enum Piece {
    King,
    Queen,
    Bishop,
    Rook,
    Knight,
    Pawn
}

struct Square {
    piece:Option<Piece>
}

impl Square {
    pub fn new() -> Square {
        Square {piece:None}
    }

    fn symbol(&self) -> &str {
        match self.piece {
            Some(Piece::King) => "K",
            Some(Piece::Queen) => "Q",
            Some(Piece::Bishop) => "B",
            Some(Piece::Rook) => "R",
            Some(Piece::Knight) => "K",
            Some(Piece::Pawn) => "P",
            None => " "
        }
    }
}

fn main() {
    println!("Hello, world!");
}
