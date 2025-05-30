use macroquad::texture::Texture2D;

use super::{Piece, PieceType, Side};
use crate::{
    path::{Direction, Magnitude, Path},
    textures::PieceTxts,
};

pub fn pawn(side: Side, txts: &'static PieceTxts) -> Piece {
    const LINE_OF_SIGHT_WHITE: [Path; 2] = [
        Path {
            direction: Direction::UpLeft,
            magnitude: Magnitude::Fixed(1),
        },
        Path {
            direction: Direction::UpRight,
            magnitude: Magnitude::Fixed(1),
        },
    ];
    const MOVESET_WHITE: [Path; 1] = [Path {
        direction: Direction::Up,
        magnitude: Magnitude::Fixed(1),
    }];
    const LINE_OF_SIGHT_BLACK: [Path; 2] = [
        Path {
            direction: Direction::DownRight,
            magnitude: Magnitude::Fixed(1),
        },
        Path {
            direction: Direction::DownLeft,
            magnitude: Magnitude::Fixed(1),
        },
    ];
    const MOVESET_BLACK: [Path; 1] = [Path {
        direction: Direction::Down,
        magnitude: Magnitude::Fixed(1),
    }];
    let txt: &Texture2D;
    let line_of_sight: &[Path; 2];
    let moveset: &[Path; 1];

    if side == Side::White {
        txt = &txts.pawn_w;
        line_of_sight = &LINE_OF_SIGHT_WHITE;
        moveset = &MOVESET_WHITE;
    } else {
        txt = &txts.pawn_b;
        line_of_sight = &LINE_OF_SIGHT_BLACK;
        moveset = &MOVESET_BLACK;
    };
    Piece {
        name: "Pawn".to_string(),
        side,
        piece_type: PieceType::Pawn,
        txt,
        line_of_sight,
        moveset,
        same_line_of_sight_and_moveset: false,
        prev_cell: None,
        last_played_move: None,
    }
}
