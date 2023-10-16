#[derive(Clone, Copy, Debug,Eq,PartialEq,Hash)]
pub enum CellType {
    Empty,
    Tree,
    Amazon,
    Threatened,
}

impl CellType {
    fn is_piece(&self) -> bool {
        match self {
            CellType::Empty => false,
            CellType::Tree => true,
            CellType::Amazon => true,
            CellType::Threatened => false
        }
    }
}

pub enum Delta{
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast
}

impl Delta{
    fn vector(&self) -> (isize, isize){
        match self {
            Delta::North => (0, -1),
            Delta::NorthWest => (-1, -1),
            Delta::West => (-1, 0),
            Delta::SouthWest => (-1, 1),
            Delta::South => (0, 1),
            Delta::SouthEast => (1, 1),
            Delta::East => (1, 0),
            Delta::NorthEast => (1, -1),
        }
    }
}

const BOARD_SIZE: usize = 4;

pub struct Board{
    cells : [ [CellType; BOARD_SIZE]; BOARD_SIZE],
}

impl Board{

    fn width(&self) -> usize{
        self.cells[0].len()
    }

    fn height(&self) -> usize{
        self.cells.len()
    }

    fn inside(&self, x: isize, y: isize ) -> bool {
        x >= 0 &&
            x < self.width() as isize &&
            y > 0 &&
            y < self.height() as isize
    }


    fn get_cell( &self, x: isize, y: isize ) -> CellType {
        if self.inside(x,y) {
            self.cells[y as usize][x as usize]
        }
        else {
            CellType::Empty
        }
    }

    fn set_cell( &mut self, x: isize, y: isize, cell_type: CellType ) {
        if !self.inside(x,y) {
            panic!();
        }
        else {
            self.cells[y as usize][x as usize] = cell_type;
        }
    }

    fn find_piece(&self, from_x: isize, from_y: isize, delta: Delta ) -> Option<(isize,isize)> {
        let delta = delta.vector();
        let mut c = (from_x, from_y);
        loop {
            c = (c.0 + delta.0, c.1 + delta.1 );
            if !self.inside( c.0, c.1 ) {
                return None;
            }
            match self.get_cell( c.0, c.1 ) {
                CellType::Tree => return Some(c),
                CellType::Amazon => return Some(c),
                _ => ()
            }
        }
        panic!();
    }

    fn add_tree( &mut self, x: isize, y: isize ){
        if self.get_cell(x,y) != CellType::Empty{
            panic!();
        }


    }
}

#[cfg(test)]
mod test{
    #[test]
    fn test(){
        println!("A test");
    }
}