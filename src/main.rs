use chess::{get_cell_width, Grid, Piece, Side};
use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    let mut cell_width = get_cell_width();
    let mut grid = Grid::new64(cell_width);
    let mut selected_cell: Option<(u32, u32)> = None;
    grid.get_cell_mut(0, 0).add(Piece {
        name: String::from("tesT"),
        side: Side::White,
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
            grid.get_cell(cell.0, cell.1).select();
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
