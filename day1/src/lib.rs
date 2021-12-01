pub fn count_depth(contents: &str) -> i32 {
    let depths = contents
        .split('\n')
        .filter_map(|line| line.parse::<i32>().ok());

    let mut nb_increases = 0;
    let mut previous_depth: Option<i32> = None;
    for current_depth in depths {
        nb_increases += match previous_depth {
            Some(previous_depth) if previous_depth < current_depth => 1,
            Some(_) => 0,
            None => 0,
        };
        previous_depth = Some(current_depth);
    }
    return nb_increases;
}


#[cfg(test)]
mod tests {
    use crate::count_depth;

    #[test]
    fn can_count_increases_in_depth() {
        let contents = "199\n\
            200\n\
            208\n\
            210\n\
            200\n\
            207\n\
            240\n\
            269\n\
            260\n\
            263";
        assert_eq!(count_depth(contents), 7);
    }
}