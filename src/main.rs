use chess::{Cell, Grid};
use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    loop {
        let h = screen_height();
        let w = screen_width();
        let cell_width = if (h > w) { w / 8.0 } else { h / 8.0 };
        let grid = Grid::new64(cell_width);
        grid.draw();
        Cell::draw_cas(grid.get_cell(1, 7));
        let (x, y) = mouse_position();

        draw_circle(x, y, 9.0, GREEN);
        next_frame().await
    }
}
