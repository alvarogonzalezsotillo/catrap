use pathfinding::num_traits::abs;
use pathfinding::prelude::dfs;
use std::io::Write;

type Number = i32;
type Coordinates = (Number, Number);

pub fn find_board(board_size: Number, max_number_of_trees: usize) -> Option<Board> {
    fn step(b: &Board, max_number_of_trees: usize ) -> Option<Board>{
        let valid = |b: &&Board| {
            b.amazons.len() <= b.board_size as usize && b.trees.len() <= max_number_of_trees
        };

        if b.amazons.len() == b.board_size as usize{
            return Some(b.clone());
        }

        for child in b.fill_next_cell_with_possible().iter().filter(valid){
            match step(child, max_number_of_trees ){
                Some(b) => return Some(b),
                _ => (),
            }
        }
        None
    }
    step(&Board::new(board_size) ,max_number_of_trees )
}

pub fn find_board_minimize_trees(board_size: Number) -> Option<Board> {
    let mut max_number_of_trees = board_size*board_size;

    let mut last_found = find_board(board_size,max_number_of_trees as usize);
    if last_found.is_none(){
        return None;
    }
    while last_found.is_some() {
        max_number_of_trees = (last_found.clone().unwrap().trees.len() - 1) as Number;
        if max_number_of_trees == -1 {
            return last_found;
        }
        match find_board(board_size, max_number_of_trees as usize){
            Some(next) => {
                next.dump_stdout(&std::format!("con {} árboles ", max_number_of_trees) );
                last_found = Some(next);
            },
            None => return last_found,
        }
    }
    panic!();
}

pub struct Board {
    board_size: Number,
    amazons: Vec<Coordinates>,
    trees: Vec<Coordinates>,
    next_cell_to_fill: Option<Coordinates>,
}

impl PartialEq<Self> for Board {
    fn eq(&self, other: &Self) -> bool {
        self.board_size == other.board_size
            && self.amazons.eq(&other.amazons)
            && self.trees.eq(&other.trees)
            && self.next_cell_to_fill == other.next_cell_to_fill
    }
}

impl Eq for Board {}

impl Clone for Board {
    fn clone(&self) -> Self {
        Board {
            board_size: self.board_size,
            amazons: self.amazons.clone(),
            trees: self.trees.clone(),
            next_cell_to_fill: self.next_cell_to_fill.clone(),
        }
    }
}

impl Board {
    fn new(board_size: Number) -> Board {
        Board {
            board_size,
            amazons: vec![],
            trees: vec![],
            next_cell_to_fill: Some((0, 0)),
        }
    }

    pub fn from_lines(lines: Vec<&str>) -> Board {
        let mut ret = Board::new(lines.len() as Number);

        for str in lines.iter() {
            for c in str.chars() {
                match c {
                    'T' => ret = ret.fill_next_cell_with_tree(),
                    'A' => ret = ret.fill_next_cell_with_amazon(),
                    '.' => ret = ret.fill_next_cell_with_empty(),
                    'x' => return ret,
                    _ => (),
                }
            }
        }
        ret
    }

    fn next_coordinates(&self, c: &Coordinates) -> Option<Coordinates> {
        match *c {
            (x, y) if x == self.board_size - 1 && y == self.board_size - 1 => None,
            (x, y) if x == self.board_size - 1 => Some((0, y + 1)),
            (x, y) => Some((x + 1, y)),
        }
    }

    fn find_threat(&self) -> Option<(Coordinates,Coordinates)>{
        for amazon in self.amazons.iter(){
            match self.cell_is_threatened( amazon ){
                Some(t) => return Some( (amazon.clone(),t) ),
                _ => ()
            }
        }
        None
    }

    fn cell_is_threatened(&self, cell: &Coordinates) -> Option<Coordinates> {
        fn same_vertical(a: &Coordinates, b: &Coordinates) -> bool {
            a.0 == b.0
        }

        fn same_horizontal(a: &Coordinates, b: &Coordinates) -> bool {
            a.1 == b.1
        }

        fn same_diagonal(a: &Coordinates, b: &Coordinates) -> bool {
            let dx = a.0 - b.0;
            let dy = a.1 - b.1;
            abs(dx) == abs(dy)
        }

        fn between_number(a: Number, b: Number, c: Number) -> bool {
            (a < b && b < c) || (a > b && b > c)
        }
        fn between_vertical(a: &Coordinates, between: &Coordinates, c: &Coordinates) -> bool {
            debug_assert!(same_vertical(a, c));
            if !same_vertical(a, between) {
                return false;
            }
            between_number(a.1, between.1, c.1)
        }

        fn between_horizontal(a: &Coordinates, between: &Coordinates, c: &Coordinates) -> bool {
            debug_assert!(same_horizontal(a, c));
            if !same_horizontal(a, between) {
                return false;
            }
            between_number(a.0, between.0, c.0)
        }

        fn between_diagonal(a: &Coordinates, between: &Coordinates, c: &Coordinates) -> bool {
            debug_assert!(same_diagonal(a, c));
            same_diagonal(a, between)
                && same_diagonal(between, c)
                && between_number(a.0, between.0, c.0)
        }

        fn knight_jump(a: &Coordinates, b: &Coordinates) -> bool {
            match a {
                (x, y) if abs(x - b.0) == 1 && abs(y - b.1) == 2 => true,
                (x, y) if abs(x - b.0) == 2 && abs(y - b.1) == 1 => true,
                _ => false,
            }
        }

        fn too_close(a: &Coordinates, b: &Coordinates) -> bool {
            abs(a.0 - b.0) <= 1 && abs(a.1 - b.1) <= 1
        }

        fn threatened(a: &Coordinates, b: &Coordinates, trees: &Vec<Coordinates>) -> bool {

            if a == b{
                return false;
            }

            if knight_jump(a, b) {
                return true;
            }
            if too_close(a, b) {
                return true;
            }
            if same_horizontal(a, b) {
                if !trees.iter().any(|t| between_horizontal(a, t, b)) {
                    return true;
                }
                return false;
            }
            if same_vertical(a, b) {
                if !trees.iter().any(|t| between_vertical(a, t, b)) {
                    return true;
                }
                return false;
            }
            if same_diagonal(a, b) {
                if !trees.iter().any(|t| between_diagonal(a, t, b)) {
                    return true;
                }
                return false;
            }

            false
        }

        self.amazons
            .iter()
            .find(|a| threatened(a, cell, &self.trees))
            .cloned()

    }

    fn fill_next_cell_with_empty(&self) -> Board {
        let next = self.next_coordinates(&self.next_cell_to_fill.unwrap() );
        let next = next;
        let next_board_without_piece = {
            let mut b = self.clone();
            b.next_cell_to_fill = next;
            b
        };
        next_board_without_piece
    }

    fn fill_next_cell_with_amazon(&self) -> Board {
        let next = self.next_coordinates(&self.next_cell_to_fill.unwrap());
        let next_board_with_amazon = {
            let mut b = self.clone();
            b.amazons.push(b.next_cell_to_fill.unwrap().clone());
            b.next_cell_to_fill = next;
            b
        };
        next_board_with_amazon
    }

    fn fill_next_cell_with_tree(&self) -> Board {
        let next = self.next_coordinates(&self.next_cell_to_fill.unwrap());
        let next_board_with_tree = {
            let mut b = self.clone();
            b.trees.push(b.next_cell_to_fill.unwrap().clone());
            b.next_cell_to_fill = next;
            b
        };
        next_board_with_tree
    }

    fn fill_next_cell_with_possible(&self) -> Vec<Board> {
        let mut ret: Vec<Board> = vec![];
        if self.next_cell_to_fill.is_none(){
            return ret;
        }

        let threatened = self.cell_is_threatened(&self.next_cell_to_fill.unwrap());
        if threatened.is_none() {
            ret.push(self.fill_next_cell_with_amazon());
        }
        if self.board_size < 12  {
            ret.push(self.fill_next_cell_with_tree());
        }
        ret.push(self.fill_next_cell_with_empty());

        ret
    }

    pub fn dump_stdout(&self, prefix: &str) {
        let _ = self.dump(&mut std::io::stdout(), prefix);
    }

    pub fn dump<W: Write>(&self, output: &mut W, prefix: &str) -> std::io::Result<()> {
        for line in 0..self.board_size {
            write!(output, "{}", prefix)?;
            for column in 0..self.board_size {
                let c = (column, line);
                let mut char = ". ";
                if self.amazons.contains(&c) {
                    char = "A ";
                }
                if self.trees.contains(&c) {
                    char = "T ";
                }
                if let Some(next) = self.next_cell_to_fill{
                    if  next == c {
                        char = "x ";
                    }
                }
                write!(output, "{}", char)?;
            }
            writeln!(output)?;
        }
        writeln!(output)?;
        writeln!(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Board;
    use crate::amazonas::Number;
    use ntest::assert_true;

    #[test]
    fn first_step() {
        let b = Board::new(4);
        let next_step = b.fill_next_cell_with_possible();
        next_step.iter().for_each(|b| b.dump_stdout(""));
    }

    #[test]
    fn second_step() {
        let b = Board::new(4);
        let next_step = b.fill_next_cell_with_possible();
        next_step.iter().for_each(|b| {
            b.dump_stdout("");
            let n = b.fill_next_cell_with_possible();
            n.iter().for_each(|b| b.dump_stdout(""));
        });
    }


    #[test]
    fn from_lines_threatened() {
        #[cfg_attr(rustfmt, rustfmt::skip)]
        let lines = vec!(
            "T T T . ",
            "A . . x ",
            ". . . . ",
            ". . . . "
        );
        let board = Board::from_lines(lines);
        board.dump_stdout("");
        assert_true!(board.cell_is_threatened(&(2 as Number, 1 as Number)).is_some());
    }

    #[test]
    fn from_lines_not_threatened() {
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let lines = vec!(
            "T T T . ",
            ". . . x ",
            ". . . . ",
            ". . . . "
        );
        let board = Board::from_lines(lines);
        board.dump_stdout("");
        assert_true!(board.cell_is_threatened(&(2 as Number, 1 as Number)).is_none());
    }

    #[test]
    fn find_board_test() {
        match super::find_board(8, 2) {
            Some(b) => b.dump_stdout("Encontrado "),
            None => assert_true!(false),
        }
    }

    #[test]
    fn find_board_minimize_trees_test() {
        let boards = super::find_board_minimize_trees(8);
        boards.unwrap().dump_stdout("");
    }


    #[test]
    fn check_jaime(){
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let lines = vec!(
            ". A T . A . . .",
            ". . . . . . . A",
            ". . . . . . . .",
            "A T A . . . . .",
            ". . . . . . A .",
            ". . . . . . . .",
            ". A . . . . . .",
            ". . . . . A . .",
        );
        let b = Board::from_lines(lines);
        match b.find_threat(){
            Some(c) => {
                println!("Amenazada:{:?}", c);
                assert!(false);
            }
            _ => (),
        }

    }
}
