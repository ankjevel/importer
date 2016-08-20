extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::str;
use std::i64;
use std::error::Error;
use std::io::prelude::*;
use std::os::unix::fs::MetadataExt;
use std::string::String;
use std::fmt;
use std::path::Path;
use std::fs::{
    read_dir,
    File as fsFile
};
use std::env::{
    current_dir as current,
    home_dir as home
};

use string::{
    string_to_static_str,
    borrowed_string_to_static_str
};

const SLASH: char = '/' as char;
const TIDLE: char = '~' as char;

fn current_directory() -> String {
    current().unwrap().into_os_string().into_string().unwrap() + "/"
}

fn home_directory() -> String {
    home().unwrap().into_os_string().into_string().unwrap() + "/"
}

fn unwrap_created_date<'a>(path: &'a Path) -> String {
    let meta = &path.metadata().unwrap();
    let created: i64 = meta.ctime();

    created.to_string()
}

pub fn unwrap_path<'a>(file_path: &&str) -> &'a Path {
    let home_dir = &*home_directory();
    let current_dir = &*current_directory();

    let mut file_path_copy = String::new();
    file_path_copy.push_str(file_path);

    let mut mutable_str = String::new();
    match file_path.as_bytes()[0] as char {
        SLASH => (),
        TIDLE => {
            mutable_str.push_str(home_dir);
            file_path_copy.remove(0);
            match file_path_copy.chars().nth(0).unwrap() {
                SLASH => { file_path_copy.remove(0); },
                _ => ()
            }
        },
        _ => {
            mutable_str.push_str(current_dir);
        }
    }
    mutable_str.push_str(&file_path_copy);
    let s: &'static str = string_to_static_str(mutable_str);
    Path::new(s)
}

fn generate_md5(bytes: &[u8]) -> String {
    let data = String::from_utf8_lossy(&bytes);

    let mut hasher = Md5::new();
    hasher.input_str(&data);

    let mut output = [0; 16];
    hasher.result(&mut output);
    hasher.result_str()
}

fn get_extension<'a>(path: &'a Path) -> &'static str {
    let extension = match path.extension() {
        None => "none",
        Some(ext) => ext.to_str().unwrap()
    };

    borrowed_string_to_static_str(&&*extension.to_lowercase())
}

pub struct File {
    pub path_string: String,
    md5: String,
    extension: String,
    created: String
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {}, {}, {})", self.path_string, self.md5, self.created, self.extension)
    }
}

impl File {
    pub fn new(path: &Path) -> File {
        let extension = get_extension(&path);

        // let mut path_str = String::new();
        // path_str.push_str(&*created);
        // path_str.push_str(".");
        // path_str.push_str(&&*extension);

        let created = unwrap_created_date(&path);

        File {
            path_string: String::from(path.to_str().unwrap()),
            md5: "".to_string(),
            created: created,
            extension: String::from(borrowed_string_to_static_str(&extension))
        }
    }

    pub fn set_md5(&mut self) {
        if self.md5 != "" {
            return
        }

        let path = Path::new(&self.path_string);

        let display = path.display();
        let mut file = match fsFile::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file
        };

        let mut bytes = Vec::new();
        match file.read_to_end(&mut bytes) {
            Err(why) => panic!("can't read file {}: {}", display, why.description()),
            Ok(_) => ()
        }

        let md5 = generate_md5(&bytes);

        self.md5 = md5;
    }
}

pub struct Files {
    pub collection: Vec<File>
}

impl Files {
    pub fn new() -> Files {
        Files {
            collection: Vec::new()
        }
    }

    pub fn collection_mut(&mut self) -> &mut Vec<File> {
        self.collection.as_mut()
    }

    pub fn check(&mut self, dir: &str) {
        self.traverse(&dir);
    }

    fn push(&mut self, file: File) {
        self.collection.push(file)
    }

    fn traverse(&mut self, dir: &str) {
        let allowed_file_types = vec![
            "aae", "arw", "jpeg", "jpg",
            "mov", "mp4", "mts", "raw"
        ];

        for entry in read_dir(&dir).unwrap() {
            let s = borrowed_string_to_static_str(&entry.unwrap().path().to_str().unwrap());
            let path = Path::new(s);

            if path.is_file() == false {
                self.traverse(path.to_str().unwrap());
                continue
            }

            if !allowed_file_types.contains(&&*get_extension(&path)) {
                continue
            }

            self.push(File::new(&path))
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use file;
}
