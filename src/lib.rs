use macroquad::prelude::*;
pub struct Cell {
    pub id: (u32, u32),
    center: (f32, f32),
    origin: (f32, f32),
    color: Color,
    size: f32,
    pub item: Option<Piece>,
}

impl Cell {
    pub fn draw(&self) {
        let (x, y) = self.origin;
        draw_rectangle(x, y, self.size, self.size, self.color);
        if let Some(piece) = &self.item {
            piece.draw(self.center);
        }
    }
    pub fn highlight(&self) {
        let (x, y) = self.origin;
        draw_rectangle(x, y, self.size, self.size, Color::new(0.2, 1.0, 1.0, 0.25));
    }

    pub fn add(&mut self, piece: Piece) {
        self.item = Some(piece);
    }

    pub fn select(&self) {
        println!("{}", self.item.is_some());
        if self.item.is_some() {
            self.highlight();
        }
    }

    pub fn move_item_to(&mut self, dest: &mut Cell) {
        let piece = self.item.take();
        dest.add(piece.unwrap());
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
                    item: None,
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
            cell.origin = origin;
            cell.center = center;
            cell.size = w;
        }
    }

    pub fn get_cell(&self, x: u32, y: u32) -> &Cell {
        self.cells.get((x * 8 + y) as usize).unwrap()
    }
    pub fn get_cell_mut(&mut self, x: u32, y: u32) -> &mut Cell {
        self.cells.get_mut((x * 8 + y) as usize).unwrap()
    }

    pub fn get_cell_mut_pair(
        &mut self,
        cell1: (u32, u32),
        cell2: (u32, u32),
    ) -> (&mut Cell, &mut Cell) {
        let cell1_idx: u32 = cell1.0 * 8 + cell1.1;
        let cell2_idx: u32 = cell2.0 * 8 + cell2.1;
        let mid: u32;
        if cell1_idx < cell2_idx {
            mid = cell1_idx + 1;
            let (g1, g2) = self.cells.split_at_mut(mid as usize);
            (
                g1.get_mut((mid - 1) as usize).unwrap(),
                g2.get_mut((cell2_idx - mid) as usize).unwrap(),
            )
        } else {
            mid = cell2_idx + 1;
            let (g1, g2) = self.cells.split_at_mut(mid as usize);
            (
                g2.get_mut((cell1_idx - mid) as usize).unwrap(),
                g1.get_mut((mid - 1) as usize).unwrap(),
            )
        }
    }

    pub fn draw(&self) {
        for cell in self.cells.iter() {
            cell.draw();
        }
    }
    //
    pub fn coord_to_cell_id(&self, mouse_coords: (f32, f32)) -> Option<(u32, u32)> {
        let w = self.cell_size;
        let (xm, ym) = mouse_coords;
        let (x, y): (u32, u32) = ((ym / w).floor() as u32, (xm / w).floor() as u32);
        if (x < 8) & (y < 8) {
            Some((x, y))
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
pub enum Side {
    Black,
    White,
}
pub struct Piece {
    pub name: String,
    pub side: Side,
}

impl Piece {
    pub fn draw(&self, origin: (f32, f32)) {
        let (x, y) = origin;
        draw_circle(x, y, 4.0, GREEN);
    }
}
