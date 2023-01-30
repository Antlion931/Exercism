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
    direction: Direction,
    position: [i32; 2],
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { direction, position: [x, y] }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::East => self.direction = Direction::North,
            Direction::South => self.direction = Direction::East,
            Direction::West => self.direction = Direction::South,
        }
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.direction {
            Direction::North => self.position[1] += 1,
            Direction::East => self.position[0] += 1,
            Direction::South => self.position[1] -= 1,
            Direction::West => self.position[0] -= 1,
        }
        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            match c {
                'A' => self = self.advance(),
                'L' => self = self.turn_left(),
                'R' => self = self.turn_right(),
                x => panic!("Encounter char {}", x),
            }
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position[0], self.position[1])
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
