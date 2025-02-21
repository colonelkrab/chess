use crate::game::Game;
use crate::grid::{CellId, Grid};
use crate::input::{left_click_handler, on_selected};
use crate::piece::{Piece, PieceType, Side};
use crate::textures::PieceTxts;
use macroquad::prelude::*;
mod game;
mod grid;
mod input;
mod path;
mod piece;
mod textures;

#[macroquad::main("Chess")]
async fn main() {
    set_pc_assets_folder("assets");
    let mut cell_width = get_cell_width();
    let mut grid = Grid::new64(cell_width);
    let mut game = Game::new();
    let piecetxts = PieceTxts::default().await;
    let mut selected_cell: Option<CellId> = None;

    grid.get_cell_mut(&CellId(5, 5))
        .add_item(Piece::new(PieceType::Pawn, &piecetxts, Side::White));

    grid.get_cell_mut(&CellId(4, 4))
        .add_item(Piece::new(PieceType::King, &piecetxts, Side::Black));

    grid.get_cell_mut(&CellId(3, 2))
        .add_item(Piece::new(PieceType::Pawn, &piecetxts, Side::Black));
    grid.get_cell_mut(&CellId(4, 6)).add_item(Piece::new(
        PieceType::Bishop,
        &piecetxts,
        Side::White,
    ));
    loop {
        if cell_width != get_cell_width() {
            selected_cell = None;
            cell_width = get_cell_width();
            grid.resize(cell_width);
        }
        left_click_handler(&mut grid, &mut selected_cell, &mut game);
        grid.draw();
        if let Some(cell) = &selected_cell {
            on_selected(&mut grid, cell, &mut game);
        }

        next_frame().await
    }
}

fn get_cell_width() -> f32 {
    let h = screen_height();
    let w = screen_width();
    if h > w {
        w / 8.0
    } else {
        h / 8.0
    }
}
