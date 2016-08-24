use std::str;
use std::path::Path;
use std::fs::{read_dir, DirEntry};
use string::borrowed_string_to_static_str;

use super::file::File;
use super::dirs::{get_extension, unwrap_path};

pub struct Files {
    pub collection: Vec<File>,
}

impl Files {
    pub fn new() -> Files {
        Files { collection: Vec::new() }
    }

    pub fn collection_mut(&mut self) -> &mut Vec<File> {
        self.collection.as_mut()
    }

    pub fn check(&mut self, dir: &str) {
        self.traverse(unwrap_path(&dir).to_str().unwrap());
    }

    fn push(&mut self, file: File) {
        self.collection.push(file)
    }

    fn traverse(&mut self, dir: &str) {
        let allowed_file_types = vec!["aae", "arw", "jpeg", "jpg", "mov", "mp4", "mts", "raw"];

        for entry in read_dir(&dir).unwrap() {
            let e: DirEntry = match entry {
                Ok(s) => s,
                Err(_) => continue,
            };

            let p = e.path();
            let f = match p.to_str() {
                Some(f) => f,
                _ => continue,
            };

            let s = borrowed_string_to_static_str(f);
            let path = Path::new(s);

            if path.is_file() == false {
                self.traverse(path.to_str().unwrap());
                continue;
            }


            if !allowed_file_types.contains(&&*get_extension(&path)) {
                continue;
            }

            self.push(File::new(&path))
        }
    }
}
