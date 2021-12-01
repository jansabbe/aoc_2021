mod count_depth;
use std::{env, fs};
use std::process::exit;

fn main() {
    match read_input_file() {
        Ok(contents) => {
            let result = count_depth::execute(contents);
            println!("Solution: {}", result);
        }
        Err(Error { message }) => {
            eprintln!("Error: {}", message);
            exit(-1);
        }
    }
}

struct Error {
    message: String,
}

impl Error {
    fn no_arguments() -> Error {
        Error { message: String::from("Expected filename as first argument") }
    }

    fn cannot_read(filename: &str, error: &std::io::Error) -> Error {
        Error { message: format!("Cannot read file {} because {}", filename, error) }
    }
}

fn read_input_file() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Result::Err(Error::no_arguments());
    }

    let filename = &args[1];
    fs::read_to_string(filename).map_err(|error| -> Error { Error::cannot_read(filename, &error) })
}