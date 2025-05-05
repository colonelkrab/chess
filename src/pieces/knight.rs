use macroquad::texture::Texture2D;

use super::{Piece, PieceType, Side};
use crate::{
    grid::CellId,
    path::{Direction, Magnitude, Path},
    textures::PieceTxts,
};

pub fn knight(side: Side, txts: &'static PieceTxts, cell: CellId) -> Piece {
    let txt: &Texture2D = if side == Side::White {
        &txts.knight_w
    } else {
        &txts.knight_b
    };
    const LINE_OF_SIGHT: [Path; 8] = [
        Path {
            direction: Direction::LUpRight1,
            magnitude: Magnitude::Fixed(1),
        },
        Path {
            direction: Direction::LUpLeft1,
            magnitude: Magnitude::Fixed(1),
        },
        Path {
            direction: Direction::LDownRight1,
            magnitude: Magnitude::Fixed(1),
        },
        Path {
            direction: Direction::LDownLeft1,
            magnitude: Magnitude::Fixed(1),
        },
        Path {
            direction: Direction::LUpRight2,
            magnitude: Magnitude::Fixed(1),
        },
        Path {
            direction: Direction::LUpLeft2,
            magnitude: Magnitude::Fixed(1),
        },
        Path {
            direction: Direction::LDownRight2,
            magnitude: Magnitude::Fixed(1),
        },
        Path {
            direction: Direction::LDownLeft2,
            magnitude: Magnitude::Fixed(1),
        },
    ];
    let mut cell_history = Vec::new();
    cell_history.push(cell);
    Piece {
        name: "Knight".to_string(),
        side,
        piece_type: PieceType::Knight,
        txt,
        moveset: &LINE_OF_SIGHT,
        line_of_sight: &LINE_OF_SIGHT,
        same_line_of_sight_and_moveset: true,
        cell_history,
    }
}
