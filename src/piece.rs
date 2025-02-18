use std::vec;

use crate::{
    grid::{Cell, CellId, Grid},
    path::{Direction, Magnitude, Path},
    textures::PieceTxts,
};
use macroquad::prelude::*;

#[derive(PartialEq)]
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
pub enum PieceType {
    Pawn,
    King,
    Bishop,
}
pub struct Piece {
    pub name: String,
    pub side: Side,
    pub piece_type: PieceType,
    pub txt: Texture2D,
    pub line_of_sight: Vec<Path>,
    pub moveset: Vec<Path>,
}

impl Piece {
    pub fn new(piece_type: PieceType, txts: &PieceTxts, side: Side) -> Piece {
        match piece_type {
            PieceType::Pawn => {
                let txt: Texture2D;
                let line_of_sight: Vec<Path>;
                let moveset: Vec<Path>;
                if side == Side::White {
                    txt = txts.pawn_w.clone();
                    line_of_sight = vec![
                        Path {
                            direction: Direction::UpLeft,
                            magnitude: Magnitude::Fixed(1),
                        },
                        Path {
                            direction: Direction::UpRight,
                            magnitude: Magnitude::Fixed(1),
                        },
                    ];
                    moveset = vec![Path {
                        direction: Direction::Up,
                        magnitude: Magnitude::Fixed(1),
                    }]
                } else {
                    txt = txts.pawn_b.clone();

                    line_of_sight = vec![
                        Path {
                            direction: Direction::DownRight,
                            magnitude: Magnitude::Fixed(1),
                        },
                        Path {
                            direction: Direction::DownLeft,
                            magnitude: Magnitude::Fixed(1),
                        },
                    ];
                    moveset = vec![Path {
                        direction: Direction::Down,
                        magnitude: Magnitude::Fixed(1),
                    }]
                };
                Piece {
                    name: "Pawn".to_string(),
                    side,
                    piece_type,
                    txt,
                    line_of_sight,
                    moveset,
                }
            }
            PieceType::King => {
                let txt = if side == Side::White {
                    txts.king_w.clone()
                } else {
                    txts.king_b.clone()
                };
                let line_of_sight = vec![
                    Path {
                        direction: Direction::Left,
                        magnitude: Magnitude::Fixed(1),
                    },
                    Path {
                        direction: Direction::Right,
                        magnitude: Magnitude::Fixed(1),
                    },
                    Path {
                        direction: Direction::UpLeft,
                        magnitude: Magnitude::Fixed(1),
                    },
                    Path {
                        direction: Direction::UpRight,
                        magnitude: Magnitude::Fixed(1),
                    },
                    Path {
                        direction: Direction::DownLeft,
                        magnitude: Magnitude::Fixed(1),
                    },
                    Path {
                        direction: Direction::DownRight,
                        magnitude: Magnitude::Fixed(1),
                    },
                    Path {
                        direction: Direction::Up,
                        magnitude: Magnitude::Fixed(1),
                    },
                    Path {
                        direction: Direction::Down,
                        magnitude: Magnitude::Fixed(1),
                    },
                ];
                Piece {
                    name: "King".to_string(),
                    side,
                    piece_type,
                    txt,
                    moveset: line_of_sight.clone(),
                    line_of_sight,
                }
            }
            PieceType::Bishop => {
                let txt = if side == Side::White {
                    txts.bishop_w.clone()
                } else {
                    txts.bishop_b.clone()
                };
                let line_of_sight = vec![
                    Path {
                        direction: Direction::UpLeft,
                        magnitude: Magnitude::Any,
                    },
                    Path {
                        direction: Direction::UpRight,
                        magnitude: Magnitude::Any,
                    },
                    Path {
                        direction: Direction::DownLeft,
                        magnitude: Magnitude::Any,
                    },
                    Path {
                        direction: Direction::DownRight,
                        magnitude: Magnitude::Any,
                    },
                ];
                Piece {
                    name: "Bishop".to_string(),
                    side,
                    piece_type,
                    txt,
                    moveset: line_of_sight.clone(),
                    line_of_sight,
                }
            }
        }
    }
    pub fn draw(&self, origin: (f32, f32), size: f32) {
        let (x, y) = origin;
        draw_texture_ex(
            &self.txt,
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
        for path in &self.moveset {
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
                    valid_moves.push(current_cell);
                    continue;
                };
                if piece.side == self.side {
                    break;
                } else {
                    valid_moves.push(current_cell);
                    break;
                }
            }
        }

        valid_moves
    }
}
