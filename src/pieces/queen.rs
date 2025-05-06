use macroquad::texture::Texture2D;

use super::{Piece, PieceType, Side};
use crate::{
    path::{Direction, Magnitude, Path},
    textures::PieceTxts,
};

pub fn queen(side: Side, txts: &'static PieceTxts) -> Piece {
    let txt: &Texture2D = if side == Side::White {
        &txts.queen_w
    } else {
        &txts.queen_b
    };
    const LINE_OF_SIGHT: [Path; 8] = [
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
        name: "Queen".to_string(),
        side,
        piece_type: PieceType::Queen,
        txt,
        moveset: &LINE_OF_SIGHT,
        line_of_sight: &LINE_OF_SIGHT,
        same_line_of_sight_and_moveset: true,
        prev_cell: None,
        last_played_move: None,
    }
}
