use ntest::assert_false;
use crate::block::Block;

pub type Coordinate = i32;
pub type Point = (Coordinate, Coordinate);



pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;

#[derive(Clone)]
pub struct Stage {
    pub blocks: [[Block; WIDTH]; HEIGHT],
}

impl Stage {

    pub fn width(&self) -> usize{
        self.blocks[0].len()
    }

    pub fn height(&self) -> usize{
        self.blocks.len()
    }

    pub fn outside(&self, location: &Point) -> bool {
        let (x, y) = *location;
        x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32
    }

    pub fn block_at(&self, point: &Point) -> Block {
        if self.outside(point) {
            Block::Wall
        } else {
            let (x, y) = *point;
            let x = x as usize;
            let y = y as usize;
            self.blocks[y][x]
        }
    }

    pub fn set_block_at(&mut self, point: &Point, block: Block) {
        assert_false!(matches!(block, Block::Hero));
        let (x, y) = *point;
        let x = x as usize;
        let y = y as usize;
        self.blocks[y][x] = block;
    }
}
