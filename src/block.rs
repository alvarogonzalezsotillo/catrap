#[derive(Clone, Copy, Debug,Eq,PartialEq,Hash)]
pub enum Block {
    Wall,
    SandWall,
    FallingGhost,
    Ghost,
    Rock,
    Stair,
    Hero,
    Empty,
}

impl Default for Block {
    fn default() -> Self {
        Block::Wall
    }
}
impl Block {
    pub fn is_empty(block: Block) -> bool {
        match block {
            Block::Empty => true,
            _ => false,
        }
    }

    pub fn from_char(c: char) -> Block {
        use Block::*;
        match c {
            'W' => Wall,
            '*' => SandWall,
            'F' => FallingGhost,
            'G' => Ghost,
            'R' => Rock,
            '=' => Stair,
            ' ' => Empty,
            '@' => Hero,
            _ => panic!("Undefined character to block conversion"),
        }
    }

    pub fn to_char(&self) -> char {
        use Block::*;
        match *self {
            Wall => 'W',
            SandWall => '*',
            FallingGhost => 'F',
            Ghost => 'G',
            Rock => 'R',
            Stair => '=',
            Empty => ' ',
            Hero => '@',
        }
    }

    pub fn is_fall(block: Block) -> bool {
        match block {
            Block::FallingGhost => true,
            Block::Rock => true,
            Block::Hero => true,
            _ => false,
        }
    }

    pub fn is_hero(block: Block) -> bool {
        match block {
            Block::Hero => true,
            _ => false,
        }
    }


    pub fn is_ghost(block: Block) -> bool {
        match block {
            Block::FallingGhost => true,
            Block::Ghost => true,
            _ => false,
        }
    }
}
