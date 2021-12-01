

pub fn count_depth(contents: &str, window_size: usize) -> i32 {
    let depths = get_depths(contents);

    let mut nb_increases = 0;
    let mut previous_depth: Option<i32> = None;

    for sliding_window in depths.windows(window_size) {
        let current_depth = sliding_window.iter().sum();
        nb_increases += match previous_depth {
            Some(previous_depth) if previous_depth < current_depth => 1,
            Some(_) => 0,
            None => 0,
        };
        previous_depth = Some(current_depth);
    }

    return nb_increases;
}

fn get_depths(contents: &str) -> Vec<i32> {
    contents
        .split('\n')
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<i32>>()
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
        assert_eq!(count_depth(contents, 1), 7);
    }

    #[test]
    fn can_count_increases_in_depth_using_sliding_windows() {
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
        assert_eq!(count_depth(contents, 3), 5);
    }
}