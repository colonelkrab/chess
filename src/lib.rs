use macroquad::prelude::*;

pub struct Cell {
    id: (i32, i32),
    center: (f32, f32),
    origin: (f32, f32),
    color: Color,
    size: f32,
}

impl Cell {
    pub fn draw(&self) {
        let (x, y) = self.origin;
        draw_rectangle(x, y, self.size, self.size, self.color);
    }
    pub fn draw_cas(cell: &Cell) {
        let (x, y) = cell.center;
        draw_circle(x, y, 10.0, GREEN);
    }
}

pub struct Grid {
    cells: Vec<Cell>,
}
impl Grid {
    pub fn new64(cell_width: f32) -> Grid {
        let w = cell_width;
        let mut cells: Vec<Cell> = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let origin = (j as f32 * w, i as f32 * w);
                cells.push(Cell {
                    origin,
                    id: (j, i),
                    size: w,
                    center: (origin.0 + (w / 2.0), origin.1 + (w / 2.0)),
                    color: if ((j % 2 == 0) & (i % 2 == 0)) | ((i + j) % 2 == 0) {
                        WHITE
                    } else {
                        BLACK
                    },
                });
            }
        }
        Grid { cells }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        self.cells.get(x * 8 + y).unwrap()
    }

    pub fn draw(&self) {
        for cell in self.cells.iter() {
            cell.draw();
        }
    }
}
