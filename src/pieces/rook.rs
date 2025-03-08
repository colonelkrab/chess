use macroquad::texture::Texture2D;

use super::{Piece, PieceType, Side};
use crate::{
    path::{Direction, Magnitude, Path},
    textures::PieceTxts,
};

pub fn rook(side: Side, txts: &'static PieceTxts) -> Piece {
    let txt: &Texture2D = if side == Side::White {
        &txts.rook_w
    } else {
        &txts.rook_b
    };
    const LINE_OF_SIGHT: [Path; 4] = [
        Path {
            direction: Direction::Up,
            magnitude: Magnitude::Any,
        },
        Path {
            direction: Direction::Down,
            magnitude: Magnitude::Any,
        },
        Path {
            direction: Direction::Left,
            magnitude: Magnitude::Any,
        },
        Path {
            direction: Direction::Right,
            magnitude: Magnitude::Any,
        },
    ];
    Piece {
        name: "Rook".to_string(),
        side,
        piece_type: PieceType::Rook,
        txt,
        moveset: &LINE_OF_SIGHT,
        line_of_sight: &LINE_OF_SIGHT,
        same_line_of_sight_and_moveset: true,
    }
}
