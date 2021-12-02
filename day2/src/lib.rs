mod movement;
mod position;

use movement::Movement;
use position::Position;

pub fn track_submarine(contents: &str) -> i32 {
    let movements = get_movements(contents);
    let final_position = movements
        .fold(Position::start(), |position, movement| position.apply(&movement));
    final_position.result()
}

fn get_movements<'a>(contents: &'a str) -> impl Iterator<Item=Movement> + 'a {
    contents
        .lines()
        .filter_map(|line| line.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_track_submarine() {
        let contents = "forward 5\n\
            down 5\n\
            forward 8\n\
            up 3\n\
            down 8\n\
            forward 2\n";
        assert_eq!(track_submarine(contents), 900);
    }
}