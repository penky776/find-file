use std::process::{Command, Output};
use std::{fmt, fs, fs::DirEntry, io};

#[derive(Debug)]
enum Error {
    DirectoryNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DirectoryNotFound => write!(f, "Could not find directory"),
        }
    }
}

fn main() {
    let mut dir = String::new();
    println!("enter directory: ");
    io::stdin()
        .read_line(&mut dir)
        .expect("could not read input");

    let dir = dir
        .trim()
        .parse::<String>()
        .expect("Could not parse directory");

    let mut input = String::new();
    println!("enter a sentence, or a part of a sentence, that you would like to query for: "); // Be warned of case sensitivity.
    io::stdin()
        .read_line(&mut input)
        .expect("could not read input");

    let input = input
        .trim()
        .parse::<String>()
        .expect("Could not parse input");

    let text = match_input(dir, &input).unwrap();
    println!("{:?}", std::str::from_utf8(&text.stdout).unwrap());
}

// TODO
fn match_input(dir: String, input: &String) -> Result<Output, Error> {
    let text = Command::new("pdfgrep")
        .args([input, &dir, "-n", "-r", "-H"])
        .output()
        .expect("Could not load document");

    return Ok(text);
}
