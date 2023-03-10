use lopdf::Document;
use std::fmt;
use std::fs;
use std::fs::DirEntry;
use std::io;

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

    let find_file = match_input(dir, &input);
    match find_file {
        Ok(matches) => {
            if matches.len() == 0 {
                println!("No matches found!");
            } else {
                for item in matches {
                    println!("page {} in {}", item.0, item.1);
                }
            }
        }
        Err(e) => println!("{}", e),
    }
}

fn match_input(dir: String, input: &String) -> Result<Vec<(u32, String)>, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => Ok(entries),
        Err(_) => Err(Error::DirectoryNotFound),
    };

    let mut matches: Vec<(u32, String)> = Vec::new();
    for entry in entries.unwrap() {
        if let Ok(entry) = entry {
            if is_file(&entry) {
                if is_pdf(&entry) {
                    let dir_path = entry
                        .path()
                        .into_os_string()
                        .into_string()
                        .expect("Could not parse directory path"); // turns the path into string
                    let doc = Document::load(dir_path.clone()).expect("Could not load document"); // loads the document
                    for page in doc.get_pages() {
                        let text = doc
                            .extract_text(&[page.0])
                            .unwrap()
                            .replace("\n", "")
                            .replace("?Identity-H Unimplemented?", ""); // grabs text
                        if text.contains(input) {
                            matches.push((page.0, dir_path.clone())); // returns page number and directory if true
                        } else {
                            continue;
                        }
                    }
                }
            } else {
                let dir_path = entry.path().into_os_string().into_string().unwrap(); // initializes sub-directory path
                match match_input(dir_path, input) {
                    Ok(_) => continue,
                    Err(e) => return Err(e),
                } // redoes the function with the sub-directory
            }
        }
    }

    return Ok(matches);
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
