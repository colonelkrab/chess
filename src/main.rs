use chess::{get_cell_width, Grid, Piece, PieceType, Side};
use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    set_pc_assets_folder("assets");
    let pawn_txt: Texture2D = load_texture("pieces_basic/white-pawn.png").await.unwrap();
    pawn_txt.set_filter(FilterMode::Linear);
    build_textures_atlas();
    let mut cell_width = get_cell_width();
    let mut grid = Grid::new64(cell_width);
    let mut selected_cell: Option<(u32, u32)> = None;
    grid.get_cell_mut(5, 5).add(Piece {
        name: String::from("tesT"),
        side: Side::White,
        piece_type: PieceType::Pawn,
        valid_moves: None,
        txt: pawn_txt,
    });
    loop {
        if cell_width != get_cell_width() {
            selected_cell = None;
            cell_width = get_cell_width();
            grid.resize(cell_width);
        }
        left_click_handler(&mut grid, &mut selected_cell);
        grid.draw();
        if let Some(cell) = selected_cell {
            on_selected(&mut grid, cell);
        }

        next_frame().await
    }
}

fn left_click_handler(grid: &mut Grid, selected_cell: &mut Option<(u32, u32)>) {
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
                let (selected_c, dest_c) = grid.get_cell_mut_pair(*selected, dest);
                selected_c.move_item_to(dest_c);
                *selected_cell = None;
            }
        }
        None => {
            if is_mouse_button_pressed(MouseButton::Left) {
                let Some(cell) = grid.coord_to_cell_id(mouse_position()) else {
                    return;
                };
                if grid.get_cell(cell.0, cell.1).item.is_none() {
                    return;
                };

                *selected_cell = Some(cell);
            }
        }
    }
}

fn on_selected(grid: &mut Grid, cell_: (u32, u32)) {
    let cell = grid.get_cell(cell_.0, cell_.1);
    let Some(piece) = &cell.item else { return };

    let Some(valid_moves) = &cell.valid_moves else {
        let valid_moves_ = piece.calc_valid_moves(cell, grid, (cell_.0, cell_.1));
        grid.get_cell_mut(cell_.0, cell_.1)
            .add_valid_moves(valid_moves_);
        return;
    };

    for valid_move in valid_moves {
        grid.get_cell(valid_move.0, valid_move.1).highlight();
    }
    cell.highlight();
}
