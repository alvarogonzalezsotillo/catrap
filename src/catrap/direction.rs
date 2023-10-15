use std::slice::Iter;
use super::stage::Point;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn move_point(&self, pos: &Point) -> Point {
        use Direction::*;
        match self {
            Up => (pos.0, pos.1 - 1),
            Down => (pos.0, pos.1 + 1),
            Left => (pos.0 - 1, pos.1),
            Right => (pos.0 + 1, pos.1),
        }
    }

    pub fn is_horizontal(&self) -> bool{
        use Direction::*;
        match self{
            Up => false,
            Down=> false,
            _ => true
        }
    }

    pub fn iterator() -> Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];
        DIRECTIONS.iter()
    }
}