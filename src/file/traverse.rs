use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

pub fn by_path<'a>(path: &'a str, files: &mut Vec<&Path>) {
    for entry in read_dir(&path).unwrap().into_iter() {
        let dir = entry.unwrap().path();
        let path = Path::new(&dir);
        
        if path.is_file() {
            files.push(&path);
        } else {
            by_path(path.to_str().unwrap(), files);
        }
    }   
}