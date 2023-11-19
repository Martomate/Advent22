
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Down,
    Up,
    Left,
}

impl Direction {
    fn opposite(self) -> Direction {
        use Direction::*;

        match self {
            Down => Up,
            Up => Down,
            Right => Left,
            Left => Right,
        }
    }

    fn turn_cw(self) -> Direction {
        use Direction::*;

        match self {
            Down => Left,
            Up => Right,
            Right => Down,
            Left => Up,
        }
    }

    fn turn_ccw(self) -> Direction {
        use Direction::*;

        match self {
            Down => Right,
            Up => Left,
            Right => Up,
            Left => Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub dir: Direction,
}

impl Player {
    pub fn new(x: i32, y: i32, dir: Direction) -> Player {
        Player { x, y, dir }
    }

    pub fn after_move(&self) -> Player {
        match self.dir {
            Direction::Down => Player {
                y: self.y + 1,
                ..*self
            },
            Direction::Up => Player {
                y: self.y - 1,
                ..*self
            },
            Direction::Right => Player {
                x: self.x + 1,
                ..*self
            },
            Direction::Left => Player {
                x: self.x - 1,
                ..*self
            },
        }
    }

    pub fn after_cw_turn(&self) -> Self {
        Player {
            dir: self.dir.turn_cw(),
            ..*self
        }
    }

    pub fn after_ccw_turn(&self) -> Self {
        Player {
            dir: self.dir.turn_ccw(),
            ..*self
        }
    }

    pub fn after_u_turn(&self) -> Self {
        Player {
            dir: self.dir.opposite(),
            ..*self
        }
    }

    pub fn rotate_cw(&self, size: u32) -> Self {
        Self::new(size as i32 - 1 - self.y, self.x, self.dir.turn_cw())
    }
    
    pub fn translate(&self, size: u32, drx: i32, dry: i32) -> Self {
        Self::new(self.x + drx * size as i32, self.y + dry * size as i32, self.dir)
    }
}
