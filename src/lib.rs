use macroquad::prelude::*;

pub struct Cell {
    id: (u32, u32),
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
    pub fn highlight(&self) {
        let (x, y) = self.origin;
        draw_rectangle(x, y, self.size, self.size, Color::new(0.2, 1.0, 1.0, 0.25));
    }
}

pub struct Grid {
    cells: Vec<Cell>,
    cell_size: f32,
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
        Grid {
            cells,
            cell_size: w,
        }
    }

    pub fn resize(&mut self, cell_width: f32) {
        let w = cell_width;
        self.cell_size = w;
        for cell in self.cells.iter_mut() {
            let (x, y) = &cell.id;

            let origin = (*x as f32 * w, *y as f32 * w);
            let center = (origin.0 + (w / 2.0), origin.1 + (w / 2.0));
            *cell = Cell {
                origin,
                size: w,
                center,
                ..*cell
            }
        }
    }

    pub fn get_cell(&self, x: u32, y: u32) -> &Cell {
        self.cells.get((x * 8 + y) as usize).unwrap()
    }

    pub fn draw(&self) {
        for cell in self.cells.iter() {
            cell.draw();
        }
    }
    //
    pub fn coord_to_cell(&self, mouse_coords: (f32, f32)) -> Option<&Cell> {
        let w = self.cell_size;
        let (xm, ym) = mouse_coords;
        let (x, y): (u32, u32) = ((ym / w).floor() as u32, (xm / w).floor() as u32);
        if (x < 8) & (y < 8) {
            Some(self.get_cell(x, y))
        } else {
            None
        }
    }
}

pub fn get_cell_width() -> f32 {
    let h = screen_height();
    let w = screen_width();
    if h > w {
        w / 8.0
    } else {
        h / 8.0
    }
}
