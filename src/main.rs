use chess::{get_cell_width, CellId, Game, Grid, Piece, PieceTxts, PieceType, Side};
use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    set_pc_assets_folder("assets");
    let mut cell_width = get_cell_width();
    let mut grid = Grid::new64(cell_width);
    let mut game = Game::new();
    let piecetxts = PieceTxts::default().await;
    let mut selected_cell: Option<CellId> = None;
    let test1 = CellId(5, 5);

    grid.get_cell_mut(&test1).add_item(Piece {
        name: String::from("white_pawn"),
        side: Side::White,
        piece_type: PieceType::Pawn,
        valid_moves: None,
        txt: piecetxts.pawn_w,
    });

    grid.get_cell_mut(&CellId(1, 2)).add_item(Piece {
        name: String::from("black_pawn"),
        side: Side::Black,
        piece_type: PieceType::Pawn,
        valid_moves: None,
        txt: piecetxts.pawn_b,
    });

    grid.get_cell_mut(&CellId(3, 7)).add_item(Piece {
        name: String::from("black_king"),
        side: Side::Black,
        piece_type: PieceType::King,
        valid_moves: None,
        txt: piecetxts.king_b,
    });
    loop {
        if cell_width != get_cell_width() {
            selected_cell = None;
            cell_width = get_cell_width();
            grid.resize(cell_width);
        }
        left_click_handler(&mut grid, &mut selected_cell, &mut game);
        grid.draw();
        if let Some(cell) = &selected_cell {
            on_selected(&mut grid, cell);
        }

        next_frame().await
    }
}

fn left_click_handler(grid: &mut Grid, selected_cell: &mut Option<CellId>, game: &mut Game) {
    match selected_cell {
        Some(selected) => {
            if is_mouse_button_pressed(MouseButton::Left) {
                let Some(dest) = grid.coord_to_cell_id(mouse_position()) else {
                    return;
                };
                if dest == *selected {
                    *selected_cell = None;
                    return;
                }
                let (selected_c, dest_c) = grid.get_cell_mut_pair(selected, &dest);
                selected_c.move_item_to(dest_c, game);
                *selected_cell = None;
            }
        }
        None => {
            if is_mouse_button_pressed(MouseButton::Left) {
                let Some(cell) = grid.coord_to_cell_id(mouse_position()) else {
                    return;
                };
                if grid.get_cell(&cell).item.is_none() {
                    return;
                };

                *selected_cell = Some(cell);
            }
        }
    }
}

fn on_selected(grid: &mut Grid, cell_id: &CellId) {
    let cell = grid.get_cell(cell_id);
    let Some(piece) = &cell.item else { return };

    let Some(valid_moves) = &cell.valid_moves else {
        let valid_moves_ = piece.calc_valid_moves(cell, grid);
        grid.get_cell_mut(cell_id).add_valid_moves(valid_moves_);
        return;
    };

    for valid_move in valid_moves {
        grid.get_cell(valid_move).highlight();
    }
    cell.highlight();
}
