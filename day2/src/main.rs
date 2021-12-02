use std::{env, fs};
use std::process::exit;
use day2::track_submarine;

fn main() {
    let result = match parse_arguments(env::args()) {
        Ok(contents) => track_submarine(&contents),
        Err(error) => {
            eprintln!("Error: {:?}", error);
            exit(1);
        }
    };

    println!("Solution: {}", result);
}

#[derive(Debug)]
enum Error {
    NoFilename,
    CannotRead
}

fn parse_arguments(mut args: env::Args) -> Result<String, Error> {
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err(Error::NoFilename),
    };

    let contents = match fs::read_to_string(&filename) {
        Ok(arg) => arg,
        Err(_) => return Err(Error::CannotRead)
    };

    Ok(contents)
}
