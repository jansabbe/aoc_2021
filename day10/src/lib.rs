pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(illegal_character)
        .map(score_illegal)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let scores: Vec<i64> = input
        .lines()
        .filter(|line| matches!(illegal_character(line), None))
        .map(autocomplete_character)
        .map(score_autocomplete)
        .collect();
    statistical::median(&scores)
}

fn autocomplete_character(line: &str) -> Vec<char> {
    let mut parens = Vec::new();

    for char_to_check in line.chars() {
        match char_to_check {
            '(' | '[' | '{' | '<' => {
                parens.push(char_to_check);
            }
            ')' | ']' | '}' | '>' => {
                parens.pop();
            }
            _ => {}
        }
    }

    parens
        .iter()
        .rev()
        .map(|char_to_complete| match char_to_complete {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => '?',
        })
        .collect()
}

fn illegal_character(line: &str) -> Option<char> {
    let mut parens = Vec::new();

    for char_to_check in line.chars() {
        match char_to_check {
            '(' | '[' | '{' | '<' => parens.push(char_to_check),
            ')' => {
                if !matches!(parens.pop(), Some('(')) {
                    return Some(char_to_check);
                }
            }
            ']' => {
                if !matches!(parens.pop(), Some('[')) {
                    return Some(char_to_check);
                }
            }
            '}' => {
                if !matches!(parens.pop(), Some('{')) {
                    return Some(char_to_check);
                }
            }
            '>' => {
                if !matches!(parens.pop(), Some('<')) {
                    return Some(char_to_check);
                }
            }
            _ => {}
        }
    }
    None
}

fn score_illegal(illegal_char: Option<char>) -> i32 {
    match illegal_char {
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        Some(_) | None => 0,
    }
}

fn score_autocomplete(completion: Vec<char>) -> i64 {
    completion.iter().fold(0, |total_score, character| {
        (total_score * 5)
            + match character {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        assert_eq!(illegal_character(line), Some('}'));
    }

    #[test]
    fn it_works_part_1() {
        let input = "\
        [({(<(())[]>[[{[]{<()<>>\n\
        [(()[<>])]({[<{<<[]>>(\n\
        {([(<{}[<>[]}>{[]{[(<()>\n\
        (((({<>}<{<{<>}{[]{[]{}\n\
        [[<[([]))<([[{}[[()]]]\n\
        [{[{({}]{}}([{[{{{}}([]\n\
        {<[[]]>}<{[{[{[]{()[[[]\n\
        [<(<(<(<{}))><([]([]()\n\
        <{([([[(<>()){}]>(<<{{\n\
        <{([{{}}[<[[[<>{}]]]>[]]\n";

        assert_eq!(part1(input), 26397)
    }

    #[test]
    fn real_part1() {
        let input = include_str!("../input");
        assert_eq!(part1(input), 364389)
    }

    #[test]
    fn it_works_part2() {
        let completed = autocomplete_character("[({(<(())[]>[[{[]{<()<>>");
        assert_eq!(completed, vec!['}', '}', ']', ']', ')', '}', ')', ']'])
    }

    #[test]
    fn it_works_part2_score() {
        let completed = autocomplete_character("[({(<(())[]>[[{[]{<()<>>");
        assert_eq!(score_autocomplete(completed), 288957)
    }

    #[test]
    fn it_works_part2_input() {
        let input = "\
        [({(<(())[]>[[{[]{<()<>>\n\
        [(()[<>])]({[<{<<[]>>(\n\
        {([(<{}[<>[]}>{[]{[(<()>\n\
        (((({<>}<{<{<>}{[]{[]{}\n\
        [[<[([]))<([[{}[[()]]]\n\
        [{[{({}]{}}([{[{{{}}([]\n\
        {<[[]]>}<{[{[{[]{()[[[]\n\
        [<(<(<(<{}))><([]([]()\n\
        <{([([[(<>()){}]>(<<{{\n\
        <{([{{}}[<[[[<>{}]]]>[]]\n";

        assert_eq!(part2(input), 288957)
    }

    #[test]
    fn real_part2() {
        let input = include_str!("../input");
        assert_eq!(part2(input), 2870201088)
    }
}
