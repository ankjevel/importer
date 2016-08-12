use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};
mod traverse;

pub struct Files<'lifetime> {
    dir:  &'lifetime str,
    paths: Vec<&'lifetime Path>
}

impl<'lifetime> Files<'lifetime> {
    pub fn new() -> Files<'lifetime> {
        Files {
            dir: "",
            paths: Vec::new()
        }
    }

    pub fn check(&mut self, dir: &'lifetime str) {
        self.set_dir(&dir.to_owned());
        self.traverse();
    }

    fn set_dir(&mut self, dir: &'lifetime str) {
        self.dir = dir;
    }
    
    fn traverse(&mut self) {
        for entry in read_dir(self.dir).unwrap().into_iter() {
            let dir = entry.unwrap().path();
            let path = Path::new(&dir);
            
            if path.is_file() {
                self.add_path(&path);
            } else {
                self.traverse(path.to_str().unwrap());
            }
        }   
    }

    fn add_path(&mut self, p: &'lifetime Path) {
        self.paths.push(&p)
    }
}

// impl<'lifetime> Iterator for Files<'lifetime> {
//     type Item = Vec<&'lifetime Path>;

//     fn next(&mut self) -> Option<Vec<&'lifetime Path>> {
//         ""
//     }
// }

// pub fn files_in_path<'lifetime>(path: &'lifetime str) -> Vec<&'lifetime Path> {
//     let mut files: Vec<&Path> = Vec::new();
//     traverse::by_path(&path, &mut files);
    
//     files
// }