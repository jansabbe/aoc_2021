mod domain;
use domain::BingoBoard;

pub fn calculate_winning_score(contents: &str) -> u32 {
    let (drawn_numbers, mut boards) = parse_input(contents);

    for drawn_number in drawn_numbers {
        for board in &mut boards {
            board.mark(drawn_number);
            if board.is_bingo() {
                return board.sum_unmarked_numbers() * drawn_number;
            }
        }
    }

    0
}

pub fn calculate_losing_score(contents: &str) -> u32 {
    let (drawn_numbers, mut boards) = parse_input(contents);
    let mut winning_scores:Vec<u32> = Vec::new();
    for drawn_number in drawn_numbers {
        for board in &mut boards {
            if board.is_bingo() {
                continue
            }

            board.mark(drawn_number);

            if board.is_bingo() {
                winning_scores.push(board.sum_unmarked_numbers() * drawn_number);
            }
        }
    }
    winning_scores.pop().unwrap_or(0)
}

fn parse_input(contents: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let game: Vec<&str> = contents.splitn(2, "\n\n").collect();
    let drawn_numbers: Vec<u32> = game[0].split(',').filter_map(|n| n.parse::<u32>().ok()).collect();
    let boards: Vec<BingoBoard> = game[1].split("\n\n").filter_map(|s| s.parse::<BingoBoard>().ok()).collect();
    (drawn_numbers, boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_winning_score() {
        let contents = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
            \n\
            22 13 17 11  0\n\
             8  2 23  4 24\n\
            21  9 14 16  7\n\
             6 10  3 18  5\n\
             1 12 20 15 19\n\
            \n\
             3 15  0  2 22\n\
             9 18 13 17  5\n\
            19  8  7 25 23\n\
            20 11 10 24  4\n\
            14 21 16 12  6\n\
            \n\
            14 21 17 24  4\n\
            10 16 15  9 19\n\
            18  8 23 26 20\n\
            22 11 13  6  5\n\
             2  0 12  3  7\n";
        let result = calculate_winning_score(contents);
        assert_eq!(result, 4512)
    }

    #[test]
    fn can_calculate_losing_score() {
        let contents = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
            \n\
            22 13 17 11  0\n\
             8  2 23  4 24\n\
            21  9 14 16  7\n\
             6 10  3 18  5\n\
             1 12 20 15 19\n\
            \n\
             3 15  0  2 22\n\
             9 18 13 17  5\n\
            19  8  7 25 23\n\
            20 11 10 24  4\n\
            14 21 16 12  6\n\
            \n\
            14 21 17 24  4\n\
            10 16 15  9 19\n\
            18  8 23 26 20\n\
            22 11 13  6  5\n\
             2  0 12  3  7\n";
        let result = calculate_losing_score(contents);
        assert_eq!(result, 1924)
    }

}