use std::{
    fs::{read_dir, DirEntry},
    path::Path,
    str,
};

use super::{
    dirs::{get_extension, unwrap_path},
    file::File,
};

pub struct Files {
    pub collection: Vec<File>,
}

impl Files {
    pub fn new() -> Files {
        Files {
            collection: Vec::new(),
        }
    }

    pub fn collection_mut(&mut self) -> &mut Vec<File> {
        self.collection.as_mut()
    }

    pub fn check(&mut self, dir: &str) {
        let dir = unwrap_path(&dir);
        self.traverse(&dir);
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

            let s = f.to_string();
            let path = Path::new(&s);

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
