mod domain;

use domain::Board;
use domain::Line;


pub fn get_number_of_overlapping_lines(contents: &str) -> usize {
    let mut board = Board::new();
    let lines: Vec<Line> = contents.lines().filter_map(|line| line.parse().ok()).collect();
    for line in lines {
        board.draw(line);
    }
    board.count_intersections()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_number_of_overlapping_lines() {
        let contents = "0,9 -> 5,9\n\
        8,0 -> 0,8\n\
        9,4 -> 3,4\n\
        2,2 -> 2,1\n\
        7,0 -> 7,4\n\
        6,4 -> 2,0\n\
        0,9 -> 2,9\n\
        3,4 -> 1,4\n\
        0,0 -> 8,8\n\
        5,5 -> 8,2\n";

        assert_eq!(get_number_of_overlapping_lines(contents), 12);
    }
}