use crate::game::Game;
use crate::path::{Direction, Path};
use crate::pieces::{Piece, PieceType, Side};
use macroquad::prelude::*;

pub struct Cell {
    pub id: CellId,
    center: (f32, f32),
    origin: (f32, f32),
    color: Color,
    size: f32,
    pub item: Option<Piece>,
    pub valid_moves: Option<Vec<CellId>>,
    pub pin: Option<Path>,
}

impl Cell {
    pub fn draw(&self, flip: bool) {
        let (x, y) = self.origin;
        draw_rectangle(x, y, self.size, self.size, self.color);
        if let Some(piece) = &self.item {
            piece.draw(self.origin, self.size, flip);
        }
    }
    pub fn highlight(&self) {
        let (x, y) = self.origin;
        draw_rectangle(x, y, self.size, self.size, Color::new(0.2, 1.0, 1.0, 0.25));
    }

    pub fn add_item(&mut self, piece: Piece) {
        self.item = Some(piece);
    }

    pub fn add_valid_moves(&mut self, valid_moves: Vec<CellId>, game: &mut Game) {
        self.valid_moves = Some(valid_moves);
        game.cell_cache.push(self.id);
    }

    pub fn move_item_to(&mut self, dest: &mut Cell, game: &mut Game) -> bool {
        let Some(valid_moves) = &self.valid_moves else {
            return false;
        };

        let Some(piece) = &self.item else {
            return false;
        };
        if !valid_moves.contains(&dest.id) {
            return false;
        };
        if let Some(dest_item) = &dest.item {
            if dest_item.side == piece.side {
                return false;
            } else {
                // capture opposite piece
                game.move_to_stack(dest.item.take().unwrap());
            }
        }
        self.valid_moves = None;
        if piece.piece_type == PieceType::King {
            match piece.side {
                Side::White => {
                    game.white_king = dest.id;
                }
                Side::Black => {
                    game.black_king = dest.id;
                }
            }
        }

        let mut piece = self.item.take().unwrap();
        piece.prev_cell = Some(self.id);
        piece.last_played_move = Some(game.move_count);
        dest.add_item(piece);
        game.last_played = Some((self.id, dest.id));
        true
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)]
pub struct CellId(pub u32, pub u32);
impl CellId {
    pub fn to_vec_idx(self) -> usize {
        (self.0 + self.1 * 8) as usize
    }

    pub fn is_valid(&self) -> bool {
        (self.0 < 8) & (self.1 < 8)
    }

    pub fn try_next_cellid(&self, direction: Direction, magnitude: i32) -> Option<CellId> {
        let CellId(x, y) = self;

        let (p, q) = direction.value();
        let (p, q) = (p * magnitude, q * magnitude);
        let (x_, y_): (u32, u32) = (
            (*x as i32 + p).try_into().unwrap_or(10),
            (*y as i32 + q).try_into().unwrap_or(10),
        );
        let next = CellId(x_, y_);
        if next.is_valid() {
            Some(next)
        } else {
            None
        }
    }

    pub fn from_vec_idx(idx: usize) -> CellId {
        let y = (idx as f32 / 8.0).floor() as u32;
        let x = idx as i32 - (y * 8) as i32;
        CellId(x as u32, y)
    }
}

pub struct Grid {
    cells: Vec<Cell>,
    cell_size: f32,
}
impl Grid {
    pub fn new64(cell_width: f32) -> Grid {
        let w = cell_width;
        let mut cells: Vec<Cell> = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let origin = (j as f32 * w, i as f32 * w);
                cells.push(Cell {
                    origin,
                    id: CellId(j, i),
                    size: w,
                    center: (origin.0 + (w / 2.0), origin.1 + (w / 2.0)),
                    color: if ((j % 2 == 0) & (i % 2 == 0)) | ((i + j) % 2 == 0) {
                        WHITE
                    } else {
                        GRAY
                    },
                    item: None,
                    valid_moves: None,
                    pin: None,
                });
            }
        }
        Grid {
            cells,
            cell_size: w,
        }
    }

    pub fn resize(&mut self, cell_width: f32) {
        let w = cell_width;
        self.cell_size = w;
        for cell in self.cells.iter_mut() {
            let CellId(x, y) = &cell.id;

            let origin = (*x as f32 * w, *y as f32 * w);
            let center = (origin.0 + (w / 2.0), origin.1 + (w / 2.0));
            cell.origin = origin;
            cell.center = center;
            cell.size = w;
        }
    }

    pub fn get_cell(&self, id: &CellId) -> &Cell {
        self.cells.get(id.to_vec_idx()).unwrap()
    }
    pub fn get_cell_mut(&mut self, coords: &CellId) -> &mut Cell {
        self.cells.get_mut(coords.to_vec_idx()).unwrap()
    }
    pub fn get_cell_mut_pair(&mut self, cell1: &CellId, cell2: &CellId) -> (&mut Cell, &mut Cell) {
        let cell1_idx: usize = cell1.to_vec_idx();
        let cell2_idx: usize = cell2.to_vec_idx();
        let mid: usize;
        if cell1_idx < cell2_idx {
            mid = cell1_idx + 1;
            let (g1, g2) = self.cells.split_at_mut(mid);
            (
                g1.get_mut(mid - 1).unwrap(),
                g2.get_mut(cell2_idx - mid).unwrap(),
            )
        } else {
            mid = cell2_idx + 1;
            let (g1, g2) = self.cells.split_at_mut(mid);
            (
                g2.get_mut(cell1_idx - mid).unwrap(),
                g1.get_mut(mid - 1).unwrap(),
            )
        }
    }

    pub fn draw(&self, flip: bool) {
        for cell in self.cells.iter() {
            cell.draw(flip);
        }
    }
    pub fn coord_to_cell_id(&self, (xm, ym): (f32, f32)) -> Option<CellId> {
        let w = self.cell_size;
        let id: CellId = CellId((xm / w).floor() as u32, (ym / w).floor() as u32);
        if id.is_valid() {
            Some(id)
        } else {
            None
        }
    }
}
#[derive(PartialEq)]
pub struct GridSize {
    pub grid: f32,
    pub cell: f32,
}
