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

    let dir = dir.trim().parse::<String>().unwrap();

    let mut input = String::new();
    println!("enter a sentence, or a part of a sentence, that you would like to query for: "); // Be warned of case sensitivity.
    io::stdin()
        .read_line(&mut input)
        .expect("could not read input");

    let input = input.trim().parse::<String>().unwrap();

    let pdf_files = read_dir(dir).unwrap();

    for file in pdf_files.iter() {
        let result = match_input(file.path().into_os_string().into_string().unwrap(), &input);
        println!("{:?}", result);
    }
}

fn read_dir(dir: String) -> Result<Vec<DirEntry>, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => Ok(entries),
        Err(_) => Err(Error::DirectoryNotFound),
    };

    let mut pdf_files = Vec::with_capacity(1000); // sets limit of 1000 pdf files in one directory

    for entry in entries.unwrap() {
        if let Ok(entry) = entry {
            if is_file(&entry) {
                if is_pdf(&entry) {
                    pdf_files.push(entry)
                }
            } else {
                match read_dir(entry.path().into_os_string().into_string().unwrap()) {
                    Ok(mut new_list) => pdf_files.append(&mut new_list),
                    Err(e) => return Err(e),
                }
            }
        }
    }
    return Ok(pdf_files);
}

fn is_file(path: &DirEntry) -> bool {
    return path.path().is_file();
}

fn match_input(path: String, input: &String) -> Result<(u32, String), String> {
    let doc = Document::load(&path).unwrap();
    for page in doc.get_pages() {
        let text = doc
            .extract_text(&[page.0])
            .unwrap()
            .replace("\n", "")
            .replace("?Identity-H Unimplemented?", "");
        if text.contains(input) {
            return Ok((page.0, path));
        }
    }
    Err("Match not found".to_string()) // TODO
}

fn is_pdf(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.to_lowercase().ends_with("pdf"))
}
