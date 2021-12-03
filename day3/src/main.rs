use std::{env, fs};
use std::process::exit;
use day3::{calculate_life_support_rating, calculate_power_consumption};

fn main() {
    match parse_arguments(env::args()) {
        Ok(contents) => {
            println!("Power consumption: {}", calculate_power_consumption(&contents));
            println!("life support rating: {}", calculate_life_support_rating(&contents));
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
