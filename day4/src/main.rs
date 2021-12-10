use std::{env, fs};
use std::process::exit;
use day4::{calculate_winning_score,calculate_losing_score};

fn main() {
    match parse_arguments(env::args()) {
        Ok(contents) => {
            println!("Winning score: {}", calculate_winning_score(&contents));
            println!("Winning score: {}", calculate_losing_score(&contents));
        },
        Err(error) => {
            eprintln!("Error: {:?}", error);
            exit(1);
        }
    };

}

#[derive(Debug)]
enum Error {
    NoFilename,
    CannotRead
}

fn parse_arguments(mut args: env::Args) -> Result<String, Error> {
    args.next();
    let filename = args.next().ok_or(Error::NoFilename)?;
    let contents = fs::read_to_string(&filename).ok();
    contents.ok_or(Error::CannotRead)
}
