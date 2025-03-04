use crate::game::Game;
use crate::grid::{CellId, Grid, GridSize};
use crate::input::{left_click_handler, on_selected};
use crate::pieces::{Piece, PieceType, Side};
use crate::textures::PieceTxts;
use macroquad::prelude::*;
mod game;
mod grid;
mod input;
mod path;
mod pieces;
mod textures;

#[macroquad::main("Chess")]
async fn main() {
    set_pc_assets_folder("assets");
    let mut cell_width = get_grid_size().cell;
    let mut grid = Grid::new64(cell_width);
    let mut game = Game::new();
    let box_txts: Box<PieceTxts> = Box::new(PieceTxts::default().await);
    build_textures_atlas();
    let piecetxts: &PieceTxts = Box::leak(box_txts);
    let mut selected_cell: Option<CellId> = None;

    grid.get_cell_mut(&CellId(0, 0))
        .add_item(Piece::new(PieceType::King, piecetxts, Side::White));
    grid.get_cell_mut(&CellId(5, 5))
        .add_item(Piece::new(PieceType::Pawn, piecetxts, Side::White));

    grid.get_cell_mut(&CellId(4, 4))
        .add_item(Piece::new(PieceType::King, piecetxts, Side::Black));

    grid.get_cell_mut(&CellId(3, 2))
        .add_item(Piece::new(PieceType::Pawn, piecetxts, Side::Black));
    grid.get_cell_mut(&CellId(4, 6)).add_item(Piece::new(
        PieceType::Bishop,
        piecetxts,
        Side::White,
    ));

    loop {
        let gridsize = get_grid_size();
        clear_background(BLUE);
        let cam = Camera2D {
            target: vec2(gridsize.grid / 2.0, gridsize.grid / 2.),
            zoom: vec2(2.0 / gridsize.grid, 2.0 / gridsize.grid),
            offset: vec2(0., 0.),
            rotation: 0.0,
            render_target: None,
            viewport: None,
        };
        set_camera(&cam);
        if cell_width != gridsize.cell {
            selected_cell = None;
            cell_width = gridsize.cell;
            grid.resize(cell_width);
        }
        left_click_handler(&mut grid, &mut selected_cell, &mut game, &cam);
        grid.draw();
        if let Some(cell) = &selected_cell {
            on_selected(&mut grid, cell, &mut game);
        }

        next_frame().await
    }
}

fn get_grid_size() -> GridSize {
    let h = screen_height();
    let w = screen_width();
    if h > w {
        GridSize {
            grid: w,
            cell: w / 8.0,
        }
    } else {
        GridSize {
            grid: h,
            cell: h / 8.0,
        }
    }
}
