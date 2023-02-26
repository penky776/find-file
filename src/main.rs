use lopdf::Document;
use std::fmt;
use std::fs;
use std::fs::DirEntry;
use std::io;

#[derive(Debug)]
enum Error {
    DirectoryNotFound,
    MatchNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DirectoryNotFound => write!(f, "Could not find directory"),
            Error::MatchNotFound => write!(f, "Could not find match"),
        }
    }
}

fn main() {
    let mut dir = String::new();
    println!("enter directory: ");
    io::stdin()
        .read_line(&mut dir)
        .expect("could not read input");

    let dir = dir.trim().parse::<String>().unwrap();

    let mut input = String::new();
    println!("enter a sentence, or a part of a sentence, that you would like to query for: "); // Be warned of case sensitivity.
    io::stdin()
        .read_line(&mut input)
        .expect("could not read input");

    let input = input.trim().parse::<String>().unwrap();

    let find_file = match_input(dir, &input);
    println!("{:?}", find_file);
}

fn match_input(dir: String, input: &String) -> Result<(u32, String), Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => Ok(entries),
        Err(_) => Err(Error::DirectoryNotFound),
    };

    for entry in entries.unwrap() {
        if let Ok(entry) = entry {
            if is_file(&entry) {
                if is_pdf(&entry) {
                    let dir_path = entry.path().into_os_string().into_string().unwrap(); // turns the path into string
                    let doc = Document::load(dir_path.clone()).unwrap(); // loads the document
                    for page in doc.get_pages() {
                        let text = doc
                            .extract_text(&[page.0])
                            .unwrap()
                            .replace("\n", "")
                            .replace("?Identity-H Unimplemented?", ""); // grabs text
                        if text.contains(input) {
                            return Ok((page.0, dir_path)); // returns page number and directory if true
                        } else {
                            continue;
                        }
                    }
                }
            } else {
                let dir_path = entry.path().into_os_string().into_string().unwrap();
                match match_input(dir_path, input) {
                    Ok((i, a)) => return Ok((i, a)),
                    Err(_) => continue,
                }; // redoes the function with the sub-directory
            }
        }
    }

    Err(Error::MatchNotFound)
}

fn is_file(path: &DirEntry) -> bool {
    return path.path().is_file();
}

fn is_pdf(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.to_lowercase().ends_with("pdf"))
}
