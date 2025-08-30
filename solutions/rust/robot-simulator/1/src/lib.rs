// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { position: (x, y), direction }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let mut direction = self.direction;
        match direction {
            Direction::North => direction = Direction::East,
            Direction::East => direction = Direction::South,
            Direction::South => direction = Direction::West,
            Direction::West => direction = Direction::North,
        }

        Self { position: self.position, direction }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let mut direction = self.direction;
        match direction {
            Direction::North => direction = Direction::West,
            Direction::West => direction = Direction::South,
            Direction::South => direction = Direction::East,
            Direction::East => direction = Direction::North,
        }

        Self { position: self.position, direction }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => Self { position: (self.position.0, self.position.1 + 1), direction: self.direction },
            Direction::East => Self { position: (self.position.0 + 1, self.position.1), direction: self.direction },
            Direction::South => Self { position: (self.position.0, self.position.1 - 1), direction: self.direction },
            Direction::West => Self { position: (self.position.0 - 1, self.position.1), direction: self.direction },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut result = self;

        for ch in instructions.chars() {
            match ch {
                'R' => result = result.turn_right(),
                'L' => result = result.turn_left(),
                'A' => result = result.advance(),
                _ => {}
            }
        }

        result
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position.0, self.position.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}