use day2::track_submarine;
use std::process::exit;
use std::{env, fs};

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
    CannotRead,
}

fn parse_arguments(mut args: env::Args) -> Result<String, Error> {
    args.next();
    let filename = args.next().ok_or(Error::NoFilename)?;
    let contents = fs::read_to_string(&filename).ok();
    contents.ok_or(Error::CannotRead)
}
