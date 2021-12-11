use day1::count_depth;
use std::process::exit;
use std::{env, fs};

fn main() {
    let result = match parse_arguments(env::args()) {
        Ok((contents, window_size)) => count_depth(&contents, window_size),
        Err(Error { message }) => {
            eprintln!("Error: {}", message);
            exit(1);
        }
    };

    println!("Solution: {}", result);
}

struct Error {
    message: String,
}

impl Error {
    fn no_filename() -> Error {
        Error {
            message: String::from("Expected filename as first argument"),
        }
    }

    fn cannot_read(filename: &str, error: &std::io::Error) -> Error {
        Error {
            message: format!("Cannot read file {} because {}", filename, error),
        }
    }
}

fn parse_arguments(mut args: env::Args) -> Result<(String, usize), Error> {
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err(Error::no_filename()),
    };

    let contents = match fs::read_to_string(&filename) {
        Ok(arg) => arg,
        Err(error) => return Err(Error::cannot_read(&filename, &error)),
    };

    let window_size = match args.next().and_then(|f| f.parse().ok()) {
        Some(arg) => arg,
        None => 1,
    };

    Ok((contents, window_size))
}
