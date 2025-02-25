use crate::grid::{CellId, Grid};
use crate::path::{Direction, Magnitude, Path};
use crate::piece::{Piece, Side};

pub struct BoardStatus {
    pinned_pieces: Vec<(CellId, Path)>,
    checks: Vec<Path>,
}
#[derive(Debug)]
pub struct Game {
    pub white_stack: Vec<Piece>,
    pub black_stack: Vec<Piece>,
    pub turn: Side,
    pub cell_cache: Vec<CellId>,
    pub white_king: CellId,
    pub black_king: CellId,
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
            white_king: CellId(0, 0),
            black_king: CellId(4, 4),
        }
    }

    pub fn switch_turns(&mut self, grid: &mut Grid) {
        self.turn = self.turn.switch();
        for id in &self.cell_cache {
            let cell = grid.get_cell_mut(id);
            cell.valid_moves = None;
            cell.pins = None;
        }
        self.cell_cache.clear();

        let status = self.get_board_status(grid).unwrap();
        for (id, path) in status.pinned_pieces {
            let cell = grid.get_cell_mut(&id);
            cell.pins = Some(path);
            self.cell_cache.push(id);
        }
    }
    fn get_board_status(&self, grid: &mut Grid) -> Option<BoardStatus> {
        let king_cell: CellId = match self.turn {
            Side::White => self.white_king,
            Side::Black => self.black_king,
        };
        let Some(king) = &grid.get_cell(&king_cell).item else {
            println!("{},{}", king_cell.0, king_cell.1);
            println!("no king");
            return None;
        };
        let mut pinned_pieces: Vec<(CellId, Path)> = Vec::new();
        let mut checks: Vec<Path> = Vec::new();
        let side = &king.side;
        for direction in Direction::iterator() {
            let mut temp: Option<(CellId, Path)> = None;
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
                if piece.side == *side {
                    match temp {
                        Some(_) => break,
                        None => {
                            temp = Some((current_cell, path));
                            continue;
                        }
                    }
                } else {
                    match temp {
                        Some(_) => {
                            for line_of_sight in &piece.line_of_sight {
                                if line_of_sight.is_equal_to(&path.flip()) {
                                    let Some(path_from_pinned) =
                                        &path.same_direction_subtract(&temp.unwrap().1)
                                    else {
                                        println!("path direction not same ");
                                        break;
                                    };
                                    pinned_pieces.push((temp.unwrap().0, *path_from_pinned));
                                    break;
                                }
                            }
                        }
                        None => {
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
        }
        Some(BoardStatus {
            pinned_pieces,
            checks,
        })
    }
}
