use crate::Movement;

pub struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    pub fn start() -> Position {
        Position { horizontal: 0, depth: 0 }
    }

    pub fn execute(&self, movement: &Movement) -> Self {
        match movement {
            Movement::Forward(forward) => Position { horizontal: self.horizontal + forward, ..*self },
            Movement::Up(up) => Position { depth: self.depth - up, ..*self },
            Movement::Down(down) => Position { depth: self.depth + down, ..*self },
        }
    }

    pub fn result(&self) -> i32 {
        self.horizontal * self.depth
    }
}
