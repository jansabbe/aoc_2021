use std::str::FromStr;

pub enum Movement {
    Forward(i32),
    Up(i32),
    Down(i32),
}

pub enum ParseMovementError {
    InvalidString,
    UnknownCommand,
}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (command, delta) = tokenize(line).ok_or(Self::Err::InvalidString)?;
        match command {
            "forward" => Ok(Self::Forward(delta)),
            "up" => Ok(Self::Up(delta)),
            "down" => Ok(Self::Down(delta)),
            _ => Err(Self::Err::UnknownCommand)
        }
    }
}

fn tokenize(line: &str) -> Option<(&str, i32)> {
    let (command, delta) = line.split_once(" ")?;
    let delta = delta.parse().ok()?;
    Some((command, delta))
}
