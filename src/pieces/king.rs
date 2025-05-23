use macroquad::texture::Texture2D;

use super::{Piece, PieceType, Side};
use crate::{
    path::{Direction, Magnitude, Path},
    textures::PieceTxts,
};

pub fn king(side: Side, txts: &'static PieceTxts) -> Piece {
    let txt: &Texture2D = if side == Side::White {
        &txts.king_w
    } else {
        &txts.king_b
    };
    const LINE_OF_SIGHT: [Path; 8] = [
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
        piece_type: PieceType::King,
        txt,
        moveset: &LINE_OF_SIGHT,
        line_of_sight: &LINE_OF_SIGHT,
        same_line_of_sight_and_moveset: true,
        prev_cell: None,
        last_played_move: None,
    }
}
