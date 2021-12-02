use crate::Movement;

pub struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    pub fn start() -> Position {
        Position { horizontal: 0, depth: 0, aim: 0 }
    }

    pub fn execute(&self, movement: &Movement) -> Self {
        match movement {
            Movement::Forward(forward) => Position {
                horizontal: self.horizontal + forward,
                depth: self.depth + self.aim * forward,
                ..*self
            },
            Movement::Up(up) => Position { aim: self.aim - up, ..*self },
            Movement::Down(down) => Position { aim: self.aim + down, ..*self },
        }
    }

    pub fn result(&self) -> i32 {
        self.horizontal * self.depth
    }
}
