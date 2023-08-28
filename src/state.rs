use ntest::assert_false;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use crate::block::Block;
use crate::stage::Stage;

use crate::direction::Direction;
use crate::stage::Point;

#[derive(Clone)]
struct State {
    stage: Rc<RefCell<Stage>>,
    heroes: Vec<Point>,
    ghosts_count: usize,
}

impl State {
    fn new(heroes: Vec<Point>, stage: Stage) -> State {
        let ghosts_count = {
            let mut count = 0;
            for x in 0..stage.width() {
                for y in 0..stage.height() {
                    let block = stage.block_at(&(x as i32, y as i32));
                    if Block::is_ghost(block) {
                        count += 1;
                    }
                }
            }
            count
        };

        let ret = State {
            stage: Rc::new(RefCell::new(stage)),
            heroes,
            ghosts_count,
        };

        ret
    }
}

impl State {
    fn move_hero(&mut self, hero_index: usize, to: Point) -> &mut Self {
        self.heroes[hero_index] = to;
        self
    }

    fn copy_stage(&mut self) -> &mut Self {
        let copy = self.stage.borrow().clone();
        self.stage = Rc::new(RefCell::new(copy));
        self
    }

    fn block_at(&self, location: &Point) -> Block {
        let block = self.stage.borrow().block_at(location);
        if self.heroes.contains(&location) && matches!(block, Block::Empty) {
            Block::Hero
        } else {
            self.stage.borrow().block_at(location)
        }
    }

    fn modify(&mut self, location: &Point, block: Block) -> &mut Self {
        assert_false!(matches!(block, Block::Hero));
        let previous = self.block_at(location);
        if Block::is_ghost(previous) {
            self.ghosts_count -= 1;
        }
        if Block::is_ghost(block) {
            self.ghosts_count += 1;
        }
        self.stage.borrow_mut().set_block_at(location, block);
        self
    }

    fn free_fall_column(&mut self, empty_location: &Point) -> &mut Self {
        println!("free_fall_column({:?}):", empty_location);
        if !Block::is_empty(self.block_at(empty_location)) {
            println!(
                "  free_fall_column({:?}): outside or not empty",
                empty_location
            );
            return self;
        }
        // RETURNS: the next location to check for a free fall step
        fn free_fall_step(myself: &mut State, empty_location: &Point) -> Option<Point> {
            if !Block::is_empty(myself.block_at(empty_location)) {
                println!("  free_fall_step({:?}): no está vacío", empty_location);
                return None;
            };
            let up = Direction::Up.move_point(empty_location);
            let block_up = myself.block_at(&up);
            println!("  free_fall_step: up:{:?} block_up:{:?}", up, block_up);
            if Block::is_hero(block_up) {
                let hero_index = myself.hero_index_at(&up).unwrap();
                myself.move_hero(hero_index, *empty_location);
                Some(up)
            } else if Block::is_fall(block_up) {
                myself.modify(&up, Block::Empty);
                myself.modify(empty_location, block_up);
                Some(up)
            } else {
                None
            }
        }
        let mut next_empty_location = free_fall_step(self, empty_location);
        while next_empty_location.is_some() {
            next_empty_location = free_fall_step(self, &next_empty_location.unwrap());
        }
        println!("  free_fall_column: recursión abajo",);
        self.free_fall_column(&Direction::Down.move_point(empty_location));
        self
    }

    fn free_fall_after_move(&mut self, from: &Point, to: &Point, next_to: &Point) -> &Self {
        println!("---- from ----- ");
        self.free_fall_column(from);
        println!("---- to ----- ");
        self.free_fall_column(to);
        println!("---- to down ----- ");
        self.free_fall_column(&Direction::Down.move_point(to));
        println!("---- next_to down ----- ");
        self.free_fall_column(&Direction::Down.move_point(next_to));
        self
    }

    fn apply_modifications<FN: FnOnce(&mut Self) -> ()>(
        &mut self,
        hero: &Point,
        to: &Point,
        next_to: &Point,
        function: FN,
    ) -> &mut Self {
        self.copy_stage();
        function(self);
        self.free_fall_after_move(&hero, &to, &next_to);
        self
    }

    pub fn next_state(&self, hero_index: usize, direction: Direction) -> Option<Self> {
        use crate::block::Block::*;
        assert!(hero_index < self.heroes.len());
        let hero = self.heroes[hero_index];
        let to = direction.move_point(&hero);
        let next_to = direction.move_point(&to);

        let hero_block = self.stage.borrow().block_at(&hero); // underlying block
        let to_block = self.block_at(&to);
        let next_to_block = self.block_at(&next_to);

        let mut ret = self.clone();

        match (to_block, next_to_block) {
            (Empty, _) => {
                ret.apply_modifications(&hero, &to, &next_to, |myself| {
                    myself.move_hero(hero_index, to);
                });
                Some(ret)
            }
            (SandWall, _) => {
                ret.apply_modifications(&hero, &to, &next_to, |myself| {
                    myself.modify(&to.clone(),Block::Empty).move_hero(hero_index, to);
                });
                Some(ret)
            }
            (Stair, _) => {
                if matches!(direction, Direction::Up) && matches!(hero_block, Block::Empty) {
                    None
                } else {
                    ret.apply_modifications(&hero, &to, &next_to, |myself| {
                        myself.move_hero(hero_index, to);
                    });
                    Some(ret)
                }
            }
            (Rock, Empty) => {
                ret.apply_modifications(&hero, &to, &next_to, |myself| {
                    myself.modify(&to, Empty).modify(&next_to, Rock);
                });

                Some(ret)
            }
            (ghost, _) if Block::is_ghost(ghost) => {
                ret.apply_modifications(&hero, &to, &next_to, |myself| {
                    myself.modify(&to, Empty).move_hero(hero_index, to);
                });
                Some(ret)
            }
            (Wall, _) => None,
            (Rock, b) if !Block::is_empty(b) => None,
            _ => None,
        }
    }
    pub fn reachable_states_one_move(&self) -> Vec<State> {
        (0..(self.heroes.len()))
            .flat_map(|hero_index| {
                Direction::iterator().map(move |d| self.next_state(hero_index, d.clone()))
            })
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect()
    }

    pub fn dump_stdout(&self) {
        let _ = self.dump(&mut std::io::stdout());
    }

    pub fn width(&self) -> usize {
        self.stage.borrow().width()
    }

    pub fn height(&self) -> usize {
        self.stage.borrow().height()
    }

    pub fn dump<W: Write>(&self, output: &mut W) -> std::io::Result<()> {
        for line in 0..self.height() {
            for column in 0..self.width() {
                let point = (column as i32, line as i32);
                let block = self.block_at(&point);
                let is_hero = self.heroes.contains(&point);
                let c = if !is_hero { block.to_char() } else { '@' };
                write!(output, "{}", c)?;
            }
            write!(output, "\n")?;
        }
        write!(output, "\n")?;
        write!(output, "\n")?;
        Ok(())
    }

    pub fn from_lines(lines: Vec<&str>) -> State {
        let blocks = [Default::default(); crate::stage::HEIGHT];
        let mut ret = Stage { blocks };
        let mut heroes: Vec<Point> = vec![];
        for (line, str) in lines.iter().enumerate() {
            for (column, c) in str.chars().enumerate() {
                let block = Block::from_char(c);
                let point = (column as i32, line as i32);
                match block {
                    Block::Hero => {
                        heroes.push(point);
                        ret.set_block_at(&point, Block::Empty);
                    }
                    _ => ret.set_block_at(&point, block),
                }
            }
        }
        State::new(heroes, ret)
    }
    fn hero_index_at(&self, location: &Point) -> Option<usize> {
        self.heroes.iter().position(|p| p == location)
    }
}

#[cfg(test)]
mod tests {
    use crate::block::Block;
    use crate::direction::Direction;
    use crate::state::State;

    #[test]
    fn whole_turn_around() {
        let point = (0, 0);
        let last_point = Direction::iterator().fold(point, |p, d| d.move_point(&p));
        assert_eq!(point, last_point);
    }

    #[test]
    fn next_states() {
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "WWWWWWWWW",
            "W @R  G W",
            "WWWW WWWW",
            "WWWWWWWWW"
        ];

        let state = State::from_lines(strings);
        state.dump_stdout();
        let next_states = state.reachable_states_one_move();
        assert_eq!(next_states.len(), 2);

        for s in next_states {
            s.dump_stdout();
        }
    }

    #[test]
    fn high_fall() {
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "WWWWWWWWW",
            "W  R    W",
            "W  R    W",
            "W @R    W",
            "WWWW    W",
            "W       W",
            "W       W",
            "WWWWWWWWW",
        ];
        let state = State::from_lines(strings);
        state.dump_stdout();
        let next_state = state.next_state(0, Direction::Right).unwrap();
        next_state.dump_stdout();

        assert!(matches!(next_state.block_at(&(3, 3)), Block::Rock));
        assert!(matches!(next_state.block_at(&(3, 2)), Block::Rock));
        assert!(matches!(next_state.block_at(&(3, 1)), Block::Empty));
        assert!(matches!(next_state.block_at(&(4, 6)), Block::Rock));
    }

    #[test]
    fn high_fall_2() {
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "WWWWWWWWW",
            "W  R    W",
            "W FR    W",
            "W @R    W",
            "WWWW    W",
            "W       W",
            "W       W",
            "WWWWWWWWW",
        ];
        let state = State::from_lines(strings);
        state.dump_stdout();
        let next_state = state.next_state(0, Direction::Right).unwrap();
        let next_state = next_state.next_state(0, Direction::Right).unwrap();
        let next_state = next_state.next_state(0, Direction::Right).unwrap();
        let next_state = next_state.next_state(0, Direction::Right).unwrap();
        next_state.dump_stdout();

        assert!(matches!(next_state.block_at(&(2, 3)), Block::FallingGhost));
        assert!(matches!(next_state.block_at(&(4, 4)), Block::Rock));
        assert!(matches!(next_state.block_at(&(4, 5)), Block::Rock));
        assert!(matches!(next_state.block_at(&(4, 6)), Block::Rock));
    }

    #[test]
    fn kill_ghost() {
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "WWWWWWWWW",
            "W       W",
            "W F     W",
            "W @G    W",
            "WWWW    W",
            "W       W",
            "W       W",
            "WWWWWWWWW",
        ];
        let state = State::from_lines(strings);
        state.dump_stdout();
        assert_eq!(state.ghosts_count, 2);
        let next_state = state.next_state(0, Direction::Right).unwrap();
        next_state.dump_stdout();
        assert!(matches!(next_state.block_at(&(3, 3)), Block::Hero));
        assert!(matches!(next_state.block_at(&(2, 3)), Block::FallingGhost));
        assert!(matches!(next_state.block_at(&(2, 2)), Block::Empty));
        assert_eq!(next_state.ghosts_count, 1);
    }

    #[test]
    fn hero_fall() {
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "WWWWWWWWW",
            "W @     W",
            "WWW     W",
            "WWWWWWWWW",
        ];
        let state = State::from_lines(strings);
        state.dump_stdout();
        let next_state = state.next_state(0, Direction::Right).unwrap();
        next_state.dump_stdout();
        assert!(matches!(next_state.block_at(&(3, 1)), Block::Empty));
        assert!(matches!(next_state.block_at(&(3, 2)), Block::Hero));
    }

    #[test]
    fn stair_down() {
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "WWWWWWWWW",
            "W F     W",
            "W @    W",
            "WW=W    W",
            "W =     W",
            "W       W",
            "W       W",
            "WWWWWWWWW",
        ];
        let state = State::from_lines(strings);
        state.dump_stdout();
        let next_state = state.next_state(0, Direction::Down).unwrap();
        next_state.dump_stdout();
        let next_state = next_state.next_state(0, Direction::Down).unwrap();
        next_state.dump_stdout();
        let next_state = next_state.next_state(0, Direction::Down).unwrap();
        next_state.dump_stdout();

        assert!(matches!(next_state.block_at(&(2, 2)), Block::FallingGhost));
        assert!(matches!(next_state.block_at(&(2, 6)), Block::Hero));
    }

    #[test]
    fn jump_to_stair() {
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "WWWWWWWWW",
            "W =     W",
            "W @     W",
            "WW=W    W",
            "W =     W",
            "W       W",
            "W       W",
            "WWWWWWWWW",
        ];
        let state = State::from_lines(strings);
        state.dump_stdout();
        match state.next_state(0, Direction::Up) {
            None => {}
            Some(state) => {
                state.dump_stdout();
                assert!(false);
            }
        }
    }
    #[test]
    fn into_sand() {
        #[cfg_attr(rustfmt, rustfmt::skip)]
            let strings = vec![
            "WWWWWWWWW",
            "W =     W",
            "W @**   W",
            "WW=W    W",
            "W =     W",
            "WWWWWWWWW",
        ];
        let state = State::from_lines(strings);
        state.dump_stdout();
        let next_state = state.next_state(0, Direction::Right).unwrap();
        next_state.dump_stdout();
        let next_state = next_state.next_state(0, Direction::Right).unwrap();
        next_state.dump_stdout();
        assert!(matches!(next_state.block_at(&(3, 2)), Block::Empty));
        assert!(matches!(next_state.block_at(&(4, 2)), Block::Empty));
        assert!(matches!(next_state.block_at(&(4, 4)), Block::Hero));
    }
}
