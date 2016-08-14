extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::str;
use std::error::Error;
use std::io::prelude::*;
use std::fs::{read_dir, File};
use std::string::String;
use std::path::Path;

fn generate_md5(path: &Path) -> String {
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file
    };

    let mut bytes = Vec::new();
    match file.read_to_end(&mut bytes) {
        Err(why) => panic!("can't read file {}: {}", display, why.description()),
        Ok(_) => () 
    }

    let data = String::from_utf8_lossy(&bytes);

    let mut hasher = Md5::new();
    hasher.input_str(&data);

    let mut output = [0; 16];
    hasher.result(&mut output);
    hasher.result_str()
}

pub struct Files {
    pub md5s: Vec<String>
}

impl Files {
    pub fn new() -> Files {
        Files {
            md5s: Vec::new()
        }
    }

    pub fn check(&mut self, dir: &str) {
        // println!("dir: {}", dir);
        self.traverse(&dir);
        // println!("contents {:?}", self.md5s);
    }

    fn push(&mut self, md5: String) {
        self.md5s.push(md5)
    }
    
    fn traverse(&mut self, dir: &str) {
        let allowed = vec![
            "aae", "arw", "jpeg", "jpg",
            "mov", "mp4", "mts", "raw"
        ];
        for entry in read_dir(&dir).unwrap() {
            let path_buf = entry.unwrap().path();
            let path = Path::new(&path_buf);

            if path.is_file() == false {
                self.traverse(path.to_str().unwrap());
                continue
            }

            let extension = match path.extension() {
                None => "none",
                Some(ext) => ext.to_str().unwrap()
            };

            if !allowed.contains(&&*extension.to_lowercase()) {
                // println!("extension: {}", extension);
                continue
            }
            
            self.push(generate_md5(&path))
        }
    }
}