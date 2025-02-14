use crate::piece::{Piece, Side};
pub struct Game {
    pub white_stack: Vec<Piece>,
    pub black_stack: Vec<Piece>,
}

impl Game {
    pub fn move_to_stack(&mut self, piece: Piece) {
        match piece.side {
            Side::Black => self.black_stack.push(piece),
            Side::White => self.white_stack.push(piece),
        }
    }

    pub fn new() -> Game {
        Game {
            white_stack: Vec::new(),
            black_stack: Vec::new(),
        }
    }
}
