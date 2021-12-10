use day5::get_number_of_overlapping_lines;

fn main() {
    let contents = include_str!("../input.txt");
    println!("{}", get_number_of_overlapping_lines(contents));
}
