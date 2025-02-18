use crate::grid::{Cell, CellId, Grid};
use crate::path::Direction;
use crate::piece::{Piece, Side};
pub struct Game {
    pub white_stack: Vec<Piece>,
    pub black_stack: Vec<Piece>,
    pub turn: Side,
    pub cell_cache: Vec<CellId>,
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
            turn: Side::White,
            cell_cache: Vec::new(),
        }
    }

    pub fn switch_turns(&mut self, grid: &mut Grid) {
        self.turn = self.turn.switch();
        for id in &self.cell_cache {
            let cell = grid.get_cell_mut(&id);
            cell.valid_moves = None;
        }
        self.cell_cache.clear();
    }

    // fn find_pinned(&self, grid: &mut Grid, king: &Cell) {
    //     let CellId(xk, yk) = king.id;
    //     let Some(king) = &king.item else {
    //         return;
    //     };
    //     let side = &king.side;
    //     for direction in Direction::iterator() {
    //         let (xd, yd) = direction.value();
    //         let (mut x, mut y) = (xk, yk);
    //         let mut temp: Option<&Cell>;
    //         loop {
    //             (x, y) = (
    //                 (x as i32 + xd).try_into().unwrap_or(10),
    //                 (y as i32 + yd).try_into().unwrap_or(10),
    //             );
    //             let id = CellId(x, y);
    //             if !id.is_valid() {
    //                 break;
    //             };
    //             let cell = grid.get_cell(&id);

    //             let Some(piece) = &cell.item else {
    //                 continue;
    //             };

    //             if temp.is_some() {
    //                 if piece.side == *side {
    //                     break;
    //                 } else {
    //                     if_pinned_update_state()
    //                 }
    //             } else {
    //                 if piece.side == *side {
    //                     temp = Some(cell);
    //                     continue;
    //                 } else {
    //                     if_check_update()
    //                 }
    //             }
    //         }
    //     }
    // }
}
