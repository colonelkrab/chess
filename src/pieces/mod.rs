use crate::{
    grid::{Cell, CellId, Grid},
    path::{Direction, Magnitude, Path},
    textures::PieceTxts,
};
use bishop::bishop;
use king::king;
use macroquad::prelude::*;
use pawn::pawn;
mod bishop;
mod king;
mod pawn;

#[derive(PartialEq, Debug)]
pub enum Side {
    Black,
    White,
}
impl Side {
    pub fn switch(&self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}
#[derive(PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    King,
    Bishop,
}
#[derive(Debug)]
pub struct Piece {
    pub name: String,
    pub side: Side,
    pub piece_type: PieceType,
    pub txt: &'static Texture2D,
    pub line_of_sight: &'static [Path],
    pub moveset: &'static [Path],
    pub same_line_of_sight_and_moveset: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, txts: &'static PieceTxts, side: Side) -> Piece {
        match piece_type {
            PieceType::Pawn => pawn(side, txts),
            PieceType::King => king(side, txts),
            PieceType::Bishop => bishop(side, txts),
        }
    }
    pub fn draw(&self, origin: (f32, f32), size: f32) {
        let (x, y) = origin;
        draw_texture_ex(
            self.txt,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }

    pub fn calc_valid_moves(&self, cell: &Cell, grid: &Grid) -> Vec<CellId> {
        let mut valid_moves: Vec<CellId> = Vec::new();
        let lists = [&self.moveset, &self.line_of_sight];
        let mut q = 0;
        for list in lists {
            for path in list.iter() {
                let max = match path.magnitude {
                    Magnitude::Any => 10,
                    Magnitude::Fixed(f) => f,
                };
                let mut current_cell = cell.id;
                let mut n: u32 = 0;
                while n < max {
                    n += 1;
                    let Some(id) = &current_cell.try_next_cellid(path.direction) else {
                        break;
                    };
                    current_cell = *id;
                    let Some(piece) = &grid.get_cell(id).item else {
                        if q == 0 {
                            valid_moves.push(current_cell);
                        }
                        continue;
                    };
                    if piece.side == self.side {
                        break;
                    } else {
                        if (self.same_line_of_sight_and_moveset & (q == 0)) | (q == 1) {
                            valid_moves.push(current_cell)
                        }
                        break;
                    }
                }
            }
            q += 1;
        }
        if self.piece_type == PieceType::King {
            remove_cells_in_check(&mut valid_moves, &self.side, grid);
            return valid_moves;
        };
        let Some(pin) = cell.pins else {
            return valid_moves;
        };
        let Some(pin_valid_cellids) = pin.get_cell_ids(cell.id) else {
            return valid_moves;
        };
        common_moves(&pin_valid_cellids, &valid_moves)
    }
}

fn common_moves(vec1: &[CellId], vec2: &[CellId]) -> Vec<CellId> {
    let num_vec1: Vec<usize> = vec1.iter().map(|cellid| cellid.to_vec_idx()).collect();

    let num_vec2: Vec<usize> = vec2.iter().map(|cellid| cellid.to_vec_idx()).collect();
    let mut common = [num_vec1, num_vec2].concat();
    common.sort();
    let mut common_cellids: Vec<CellId> = Vec::new();
    for (x, y) in common.iter().zip(common.iter().skip(1)) {
        if x == y {
            common_cellids.push(CellId::from_vec_idx(*x));
        }
    }
    common_cellids
}

fn remove_cells_in_check(main: &mut Vec<CellId>, side: &Side, grid: &Grid) {
    let mut remove_list: Vec<usize> = Vec::new();
    for (i, valid_cell) in main.iter().enumerate() {
        let mut continue_ = true;

        for direction in Direction::iterator() {
            if !continue_ {
                break;
            }
            let mut current_id: CellId = *valid_cell;
            let mut n = 0;

            while let Some(new_id) = current_id.try_next_cellid(*direction) {
                if !continue_ {
                    break;
                }
                current_id = new_id;
                n += 1;
                let cell = grid.get_cell(&current_id);
                let Some(piece) = &cell.item else {
                    continue;
                };

                let path = Path {
                    magnitude: Magnitude::Fixed(n),
                    direction: *direction,
                };
                if piece.side == *side {
                    break;
                } else {
                    for line_of_sight in piece.line_of_sight {
                        if line_of_sight.is_equal_to(&path.flip()) {
                            remove_list.push(i);
                            continue_ = false;
                            break;
                        }
                    }
                }
            }
        }
    }

    remove_list.reverse();
    for i in remove_list {
        main.remove(i);
    }
}
