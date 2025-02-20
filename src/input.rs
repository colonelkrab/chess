use crate::{game::Game, grid::CellId, grid::Grid};
use macroquad::prelude::*;
pub fn left_click_handler(grid: &mut Grid, selected_cell: &mut Option<CellId>, game: &mut Game) {
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
                selected_c
                    .move_item_to(dest_c, game)
                    .then(|| game.switch_turns(grid));
                *selected_cell = None;
            }
        }
        None => {
            if is_mouse_button_pressed(MouseButton::Left) {
                let Some(cell) = grid.coord_to_cell_id(mouse_position()) else {
                    return;
                };

                let Some(piece) = &grid.get_cell(&cell).item else {
                    return;
                };

                if piece.side != game.turn {
                    return;
                }

                *selected_cell = Some(cell);
            }
        }
    }
}

pub fn on_selected(grid: &mut Grid, cell_id: &CellId, game: &mut Game) {
    let cell = grid.get_cell(cell_id);
    let Some(piece) = &cell.item else { return };

    let Some(valid_moves) = &cell.valid_moves else {
        let valid_moves_ = piece.calc_valid_moves(cell, grid);
        grid.get_cell_mut(cell_id)
            .add_valid_moves(valid_moves_, game);
        return;
    };

    for valid_move in valid_moves {
        grid.get_cell(valid_move).highlight();
    }
    cell.highlight();
}
