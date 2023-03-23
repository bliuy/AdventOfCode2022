use std::{
    cell::RefCell,
    clone,
    collections::HashMap,
    error::Error,
    io::{BufRead, Read},
    rc::Rc,
};

use regex::Regex;

trait Node {
    fn get_size(&self) -> i32;
}

type MutNodeFolder = Option<Rc<RefCell<Box<Folder>>>>;
type MutNodeFile = Option<Rc<RefCell<Box<File>>>>;

struct Folder {
    name: String,
    size: i32,
    sub_folders: HashMap<String, MutNodeFolder>,
    sub_files: HashMap<String, MutNodeFile>,
    // "parent" field can be owned by multiple owners - The original node itself + the cursor variable - Hence Rc is required.
    // "parent" field requires a multiple mutable references - Hence a RefCell is required.
    // "parent" field must also be Boxed - Sized types.
    parent: MutNodeFolder,
}

impl Folder {
    fn add_folder(&mut self, folder: MutNodeFolder) {
        let name = folder.as_ref().unwrap().borrow().as_ref().name.clone(); // "folder.name" field was partially moved into the "name" variable here. As such, cloning the field is required to avoid partial move.
        self.sub_folders.insert(name, folder);
    }

    fn add_file(&mut self, file: MutNodeFile) {
        let name = file.unwrap().borrow().as_ref().name.clone();
    }
    fn create_folder(name: String, parent: MutNodeFolder) -> Self {
        Folder {
            name: name,
            size: 0,
            sub_folders: HashMap::new(),
            sub_files: HashMap::new(),
            parent: parent,
        }
    }
}

impl Node for Folder {
    fn get_size(&self) -> i32 {
        let total_folder_size: i32 = self
            .sub_folders
            .values()
            .map(|x| x.as_ref().unwrap().borrow().as_ref().size)
            .sum();
        let total_file_size: i32 = self
            .sub_files
            .values()
            .map(|x| x.as_ref().unwrap().borrow().as_ref().size)
            .sum();
        total_file_size + total_folder_size
    }
}

struct File {
    name: String,
    size: i32,
    parent: Option<Box<Folder>>,
}

impl Node for File {
    fn get_size(&self) -> i32 {
        self.size
    }
}

impl File {
    fn create_file(name: String, size: i32, parent: Option<Box<Folder>>) -> Self {
        File {
            name: name,
            size: size,
            parent: parent,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Reading the file
    let file = std::fs::File::open("inputs/input5.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Compiling the regex patterns
    let mut patterns = HashMap::new();
    patterns.insert("ls", Regex::new(r"\$ ls")?);
    patterns.insert("cd", Regex::new(r"\$ cd ([A-z0-9\/\.]+)")?);
    patterns.insert("dir", Regex::new(r"dir ([A-z0-9]+)")?);
    patterns.insert("file", Regex::new(r"([0-9]+) ([A-z0-9]+.[A-z]+)")?);

    // Creating the initial cursor
    let root_folder = Folder::create_folder(r"/".to_owned(), None);
    let mut cursor = Rc::new(RefCell::new(Box::new(root_folder)));
    for line in reader.lines() {
        let line = line?;
        if let Some(_) = patterns.get("ls").unwrap().captures(&line) {
            continue;
        } else if let Some(matched) = patterns.get("cd").unwrap().captures(&line) {
            let directory = matched
                .get(1)
                .expect("Unreachable, always contains the match group.")
                .as_str();
            match directory {
                r"/" => {
                    while let Some(original_parent) = &cursor.clone().as_ref().borrow().parent {
                        cursor = original_parent.clone();
                    }
                }
                r".." => {
                    if let Some(original_parent) = &cursor.clone().as_ref().borrow().parent {
                        cursor = original_parent.clone();
                    }
                }
                name => {
                    let cloned = cursor.clone();
                    let borrowed = cloned.borrow();
                    let updated = borrowed
                        .sub_folders
                        .get(name)
                        .expect("Invalid folder referenced.")
                        .as_ref()
                        .unwrap();
                    cursor = updated.clone();
                }
            }
        } else if let Some(matched) = patterns.get("dir").unwrap().captures(&line) {
            let directory = matched
                .get(1)
                .expect("Unreachable, always contains the match group.")
                .as_str();

            // Checking if the directory exists
            let is_exists = cursor.borrow().as_ref().sub_folders.contains_key(directory);

            if !is_exists {
                let new_folder = Folder::create_folder(directory.to_owned(), Some(cursor.clone()));
                cursor.clone().borrow_mut().as_mut().sub_folders.insert(
                    directory.to_owned(),
                    Some(Rc::new(RefCell::new(Box::new(new_folder)))),
                );
            }
        }
    }

    Ok(())
}
