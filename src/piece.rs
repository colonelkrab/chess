use crate::{grid::Cell, grid::CellId, grid::Grid, textures::PieceTxts};
use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum Side {
    Black,
    White,
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
    pub valid_moves: Option<Vec<(u32, u32)>>,
}

impl Piece {
    pub fn new(piece_type: PieceType, txts: PieceTxts, side: Side) -> Piece {
        match piece_type {
            PieceType::Pawn => {
                let txt = if side == Side::White {
                    txts.pawn_w
                } else {
                    txts.pawn_b
                };
                Piece {
                    name: "Pawn".to_string(),
                    side,
                    piece_type,
                    txt,
                    valid_moves: None,
                }
            }
            PieceType::King => {
                let txt = if side == Side::White {
                    txts.king_w
                } else {
                    txts.king_b
                };
                Piece {
                    name: "King".to_string(),
                    side,
                    piece_type,
                    txt,
                    valid_moves: None,
                }
            }
            PieceType::Bishop => {
                let txt = if side == Side::White {
                    txts.bishop_w
                } else {
                    txts.bishop_b
                };
                Piece {
                    name: "Bishop".to_string(),
                    side,
                    piece_type,
                    txt,
                    valid_moves: None,
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
        let CellId(x, y) = cell.id;
        let mut valid_moves: Vec<CellId> = Vec::new();
        match self.piece_type {
            PieceType::Pawn => match self.side {
                Side::Black => {
                    let move_id = CellId(x, y + 1);
                    if move_id.is_valid() {
                        valid_moves.push(move_id);
                    }
                }
                Side::White => {
                    if y == 0 {
                        return valid_moves;
                    }

                    let move_id = CellId(x, y - 1);
                    if move_id.is_valid() {
                        valid_moves.push(move_id);
                    }
                }
            },
            PieceType::King => {}
            PieceType::Bishop => {}
        }

        valid_moves
    }
}
