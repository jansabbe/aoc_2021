use std::str::FromStr;

pub enum Movement {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some((command, delta)) = Self::tokenize(line) {
            match command.as_str() {
                "forward" => Ok(Movement::Forward(delta)),
                "up" => Ok(Movement::Up(delta)),
                "down" => Ok(Movement::Down(delta)),
                _ => Err("Invalid command".to_string())
            }
        } else {
            Err("Could not parse line".to_string())
        }
    }
}

impl Movement {
    fn tokenize(line: &str) -> Option<(String, i32)> {
        let mut tokens = line.split_whitespace();

        let command = match tokens.next() {
            Some(command) => command,
            None => return None
        };
        let delta: i32 = match tokens.next() {
            Some(delta) => delta.parse().expect("Should always be number"),
            None => return None
        };
        Some((command.to_string(), delta))
    }
}
