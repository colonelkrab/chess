use crate::grid::{Cell, CellId, Grid};
use crate::path::{Direction, Magnitude, Path};
use crate::piece::{Piece, Side};

pub struct BoardStatus {
    pinned: Vec<CellId>,
    check: Vec<Path>,
}
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
            let cell = grid.get_cell_mut(id);
            cell.valid_moves = None;
        }
        self.cell_cache.clear();
        self.get_board_status(grid, CellId(4, 4));
    }
    fn get_board_status(&self, grid: &mut Grid, king_cell: CellId) {
        let Some(king) = &grid.get_cell(&king_cell).item else {
            return;
        };
        let mut pinned_pieces: Vec<CellId> = Vec::new();
        let mut checks: Vec<Path> = Vec::new();
        let side = &king.side;
        for direction in Direction::iterator() {
            let mut temp: Option<CellId> = None;
            let mut current_cell = king_cell;
            let mut n = 0;

            while n < 10 {
                n += 1;
                let Some(id) = &current_cell.try_next_cellid(*direction) else {
                    break;
                };

                current_cell = *id;
                let cell = grid.get_cell(id);
                let Some(piece) = &cell.item else {
                    continue;
                };
                let path = Path {
                    magnitude: Magnitude::Fixed(n),
                    direction: *direction,
                };
                if temp.is_some() {
                    if piece.side == *side {
                        break;
                    } else {
                        for line_of_sight in &piece.line_of_sight {
                            if line_of_sight.is_equal_to(&path.flip()) {
                                pinned_pieces.push(temp.unwrap());
                                break;
                            }
                        }
                    }
                } else {
                    if piece.side == *side {
                        temp = Some(current_cell);
                        continue;
                    } else {
                        for line_of_sight in &piece.line_of_sight {
                            if line_of_sight.is_equal_to(&path.flip()) {
                                checks.push(path);
                                break;
                            }
                        }
                    }
                }
            }
        }
        println!("{:?}", pinned_pieces);
        println!("{:?}", checks);
    }
}
