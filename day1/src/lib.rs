pub fn count_depth(contents: &str, window_size: usize) -> usize {
    let measurements = get_measurements(contents);
    let sliding_sums = calculate_sliding_sums(measurements, window_size);
    count_increases(&sliding_sums)
}

fn get_measurements(contents: &str) -> Vec<i32> {
    contents
        .split('\n')
        .filter_map(|line| line.parse::<i32>().ok())
        .collect()
}

fn calculate_sliding_sums(measurements: Vec<i32>, window_size: usize) -> Vec<i32> {
    measurements
        .windows(window_size)
        .map(|window| window.iter().sum())
        .collect()
}

fn count_increases(measurements: &[i32]) -> usize {
    measurements
        .windows(2)
        .filter(|&arg| match arg {
            [prev, next] if prev < next => true,
            _ => false
        })
        .count()
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