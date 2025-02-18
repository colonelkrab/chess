use self::Direction::*;
use std::slice::Iter;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
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
        }
    }

    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] =
            [Right, Left, Up, Down, UpRight, UpLeft, DownRight, DownLeft];
        DIRECTIONS.iter()
    }
}
#[derive(Clone, Copy, PartialEq)]
pub enum Magnitude {
    Fixed(u32),
    Any,
}

#[derive(Clone, Copy)]
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
        };
        Path {
            magnitude: self.magnitude,
            direction,
        }
    }

    pub fn is_equal_to(&self, other: &Path) -> bool {
        let direction: bool = self.direction == other.direction;
        let magnitude = match self.magnitude {
            Magnitude::Any => true,
            Magnitude::Fixed(_) => {
                if other.magnitude == Magnitude::Any {
                    return true;
                } else {
                    return self.magnitude == other.magnitude;
                }
            }
        };

        direction & magnitude
    }
}
