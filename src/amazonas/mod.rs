use pathfinding::num_traits::abs;
use std::io::Write;

type Number = i32;
type Coordinates = (Number, Number);

fn next_coordinates(c: &Coordinates, board_size: Number) -> Option<Coordinates> {
    match *c {
        (x, y) if x == board_size - 1 && y == board_size - 1 => None,
        (x, y) if x == board_size - 1 => Some((0, y + 1)),
        (x, y) => Some((x + 1, y)),
    }
}

struct Board {
    board_size: Number,
    amazons: Vec<Coordinates>,
    trees: Vec<Coordinates>,
    next_cell_to_fill: Coordinates,
}

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
            next_cell_to_fill: (0, 0),
        }
    }

    pub fn from_lines(lines: Vec<&str>) -> Board {
        let mut ret = Board::new(lines.len() as Number);

        for (line, str) in lines.iter().enumerate() {
            let mut column : i32 = 0;
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

    fn cell_is_threatened(&self, cell: &Coordinates) -> bool {
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
            if !same_horizontal( a, between ){
                return false;
            }
            between_number(a.1, between.1, c.1)
        }

        fn between_horizontal(a: &Coordinates, between: &Coordinates, c: &Coordinates) -> bool {
            debug_assert!(same_horizontal(a, c));
            if !same_horizontal( a, between ){
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
            .any(|a| threatened(a, cell, &self.trees))
    }

    fn fill_next_cell_with_empty(&self) -> Board{
        let next = next_coordinates(&self.next_cell_to_fill, self.board_size);
        let next = next.unwrap();
        let next_board_without_piece = {
            let mut b = self.clone();
            b.next_cell_to_fill = next;
            b
        };
        next_board_without_piece
    }

    fn fill_next_cell_with_amazon(&self) -> Board{
        let next = next_coordinates(&self.next_cell_to_fill, self.board_size);
        let next = next.unwrap();
        let next_board_with_amazon = {
            let mut b = self.clone();
            b.amazons.push(b.next_cell_to_fill.clone());
            b.next_cell_to_fill = next;
            b
        };
        next_board_with_amazon
    }

    fn fill_next_cell_with_tree(&self) -> Board{
        let next = next_coordinates(&self.next_cell_to_fill, self.board_size);
        let next = next.unwrap();
        let next_board_with_tree = {
            let mut b = self.clone();
            b.trees.push(b.next_cell_to_fill.clone());
            b.next_cell_to_fill = next;
            b
        };
        next_board_with_tree
    }

    fn fill_next_cell_with_possible(&self) -> Vec<Board> {
        let mut ret: Vec<Board> = vec![];
        let next = next_coordinates(&self.next_cell_to_fill, self.board_size);
        if next.is_none() {
            return ret;
        }

        ret.push(self.fill_next_cell_with_empty());

        let threatened = self.cell_is_threatened(&self.next_cell_to_fill);
        if !threatened {
            ret.push(self.fill_next_cell_with_amazon());
        }

        ret.push(self.fill_next_cell_with_tree());

        ret
    }

    pub fn dump_stdout(&self) {
        let _ = self.dump(&mut std::io::stdout());
    }

    pub fn dump<W: Write>(&self, output: &mut W) -> std::io::Result<()> {
        for line in 0..self.board_size {
            for column in 0..self.board_size {
                let c = (column, line);
                let mut char = ". ";
                if self.amazons.contains(&c) {
                    char = "A ";
                }
                if self.trees.contains(&c) {
                    char = "T ";
                }
                if self.next_cell_to_fill == c {
                    char = "x ";
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
    use std::io::Write;
    use ntest::assert_true;
    use pathfinding::num_traits::Num;
    use crate::amazonas::Number;
    use super::Board;
    #[test]
    fn test() {
        println!("A test");
    }

    #[test]
    fn first_step() {
        let b = Board::new(4);
        let next_step = b.fill_next_cell_with_possible();
        next_step.iter().for_each(|b| b.dump_stdout());
    }

    #[test]
    fn second_step() {
        let b = Board::new(4);
        let next_step = b.fill_next_cell_with_possible();
        next_step.iter().for_each(|b| {
            b.dump_stdout();
            let n = b.fill_next_cell_with_possible();
            n.iter().for_each(|b| b.dump_stdout());
        });
    }

    #[test]
    fn some_steps() {
        
        fn step(b: &Board, level: i32) {
            let out = &mut std::io::stdout();
            if level < 0 {
                if b.amazons.len() > 3 {
                    b.dump(out);
                }
                return;
            }
            let n = b.fill_next_cell_with_possible();
            n.iter().for_each(|b| step(b, level - 1));
        }
        let b = Board::new(4);
        step(&b, 14);
    }


    #[test]
    fn from_lines_threatened(){
        #[cfg_attr(rustfmt, rustfmt::skip)]
        let lines = vec!(
            "T T T . ",
            "A . . x ",
            ". . . . ",
            ". . . . "
        );
        let board = Board::from_lines(lines);
        board.dump_stdout();
        assert_true!( board.cell_is_threatened( &(2 as Number,1 as Number) ) );
    }

    #[test]
    fn from_lines_not_threatened(){
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let lines = vec!(
            "T T T . ",
            ". . . x ",
            ". . . . ",
            ". . . . "
        );
        let board = Board::from_lines(lines);
        board.dump_stdout();
        assert_true!( !board.cell_is_threatened( &(2 as Number,1 as Number) ) );
    }

}
