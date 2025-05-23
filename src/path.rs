use self::Direction::*;
use crate::grid::CellId;
use std::slice::Iter;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    LUpRight1,
    LUpLeft1,
    LDownRight1,
    LDownLeft1,
    LUpRight2,
    LUpLeft2,
    LDownRight2,
    LDownLeft2,
}

impl Direction {
    pub fn value(&self) -> (i32, i32) {
        match self {
            Right => (1, 0),
            Left => (-1, 0),
            Up => (0, -1),
            Down => (0, 1),
            UpRight => (1, -1),
            UpLeft => (-1, -1),
            DownRight => (1, 1),
            DownLeft => (-1, 1),
            LDownRight1 => (1, -2),
            LUpRight1 => (1, 2),
            LUpLeft1 => (-1, 2),
            LDownLeft1 => (-1, -2),
            LDownRight2 => (-2, 1),
            LUpRight2 => (2, 1),
            LUpLeft2 => (2, -1),
            LDownLeft2 => (-2, -1),
        }
    }

    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 16] = [
            Right,
            Left,
            Up,
            Down,
            UpRight,
            UpLeft,
            DownRight,
            DownLeft,
            LUpRight1,
            LUpLeft1,
            LDownRight1,
            LDownLeft1,
            LUpRight2,
            LUpLeft2,
            LDownRight2,
            LDownLeft2,
        ];
        DIRECTIONS.iter()
    }
    pub fn flip(&self) -> Direction {
        match self {
            Right => Left,
            Left => Right,
            Up => Down,
            Down => Up,
            UpRight => DownLeft,
            UpLeft => DownRight,
            DownRight => UpLeft,
            DownLeft => UpRight,
            LUpRight1 => LDownLeft2,
            LUpLeft1 => LDownRight2,
            LDownRight1 => LUpLeft2,
            LDownLeft1 => LUpRight2,
            LUpRight2 => LDownLeft1,
            LUpLeft2 => LDownRight1,
            LDownRight2 => LUpLeft1,
            LDownLeft2 => LUpRight1,
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Magnitude {
    Fixed(u32),
    Any,
}

#[derive(Clone, Copy, Debug)]
pub struct Path {
    pub magnitude: Magnitude,
    pub direction: Direction,
}

impl Path {
    pub fn flip(&self) -> Path {
        let direction = match self.direction {
            Right => Left,
            Left => Right,
            Up => Down,
            Down => Up,
            UpRight => DownLeft,
            UpLeft => DownRight,
            DownRight => UpLeft,
            DownLeft => UpRight,
            LUpRight1 => LDownLeft2,
            LUpLeft1 => LDownRight2,
            LDownRight1 => LUpLeft2,
            LDownLeft1 => LUpRight2,
            LUpRight2 => LDownLeft1,
            LUpLeft2 => LDownRight1,
            LDownRight2 => LUpLeft1,
            LDownLeft2 => LUpRight1,
        };
        Path {
            magnitude: self.magnitude,
            direction,
        }
    }

    pub fn is_equal_to(&self, other: &Path) -> bool {
        let direction: bool = self.direction == other.direction;

        let magnitude: bool = match self.magnitude {
            Magnitude::Any => true,
            Magnitude::Fixed(_) => {
                if other.magnitude == Magnitude::Any {
                    true
                } else {
                    self.magnitude == other.magnitude
                }
            }
        };

        direction & magnitude
    }

    pub fn same_direction_subtract(&self, other: &Path) -> Option<Path> {
        println!("same dir sub: {:?}, {:?}", self, other);
        if self.direction != other.direction {
            return None;
        }

        let x: u32 = match self.magnitude {
            Magnitude::Fixed(n) => n,
            Magnitude::Any => return None,
        };
        let y: u32 = match other.magnitude {
            Magnitude::Fixed(n) => n,
            Magnitude::Any => return None,
        };

        let diff_i32 = (x as i32) - (y as i32);
        Some(Path {
            direction: self.direction,
            magnitude: Magnitude::Fixed(diff_i32.unsigned_abs()),
        })
    }

    pub fn get_cell_ids(self: &Path, origin: CellId) -> Option<Vec<CellId>> {
        println!("{:?}", self);
        let n: u32 = match self.magnitude {
            Magnitude::Any => {
                return None;
            }
            Magnitude::Fixed(f) => f,
        };
        let mut i = 0;
        let mut cells = Vec::new();
        let mut temp = origin;
        while i < n {
            i += 1;
            let new = temp.try_next_cellid(self.direction, 1).unwrap();
            cells.push(new);
            temp = new;
            println!("{:?}", cells);
        }
        Some(cells)
    }
}
