// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<u32> for Direction {
    fn from(n: u32) -> Direction {
        match n % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn turn_right(self) -> Self {
        Direction::from(self as u32 + 1)
    }

    fn turn_left(self) -> Self {
        Direction::from(self as u32 + 3)
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            d: self.d.turn_right(),
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            d: self.d.turn_left(),
            ..self
        }
    }

    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Robot {
                y: self.y + 1,
                ..self
            },
            Direction::East => Robot {
                x: self.x + 1,
                ..self
            },
            Direction::South => Robot {
                y: self.y - 1,
                ..self
            },
            Direction::West => Robot {
                x: self.x - 1,
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'L' => robot.turn_left(),
            'R' => robot.turn_right(),
            'A' => robot.advance(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
