use chess::{get_cell_width, Cell, Grid};
use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    let mut cell_width = get_cell_width();
    let mut grid = Grid::new64(cell_width);
    let mut highlight_cell: Option<&Cell> = None;
    loop {
        if cell_width != get_cell_width() {
            highlight_cell = None;
            cell_width = get_cell_width();
            grid.resize(cell_width);
        }
        grid.draw();
        if is_mouse_button_pressed(MouseButton::Left) {
            highlight_cell = grid.coord_to_cell(mouse_position());
        }

        if let Some(cell) = highlight_cell {
            cell.highlight();
        }

        next_frame().await
    }
}
