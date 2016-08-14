extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::str;
use std::error::Error;
use std::io::prelude::*;
use std::mem::{transmute, forget};
use std::fs::{read_dir, File};
use std::string::String;
use std::path::{Path};
use std::env::{current_dir as current, home_dir as home};

const SLASH: char = '/' as char;
const TIDLE: char = '~' as char;

fn current_directory() -> String {
    current().unwrap().into_os_string().into_string().unwrap() + "/"
}

fn home_directory() -> String {
    home().unwrap().into_os_string().into_string().unwrap() + "/"
} 

fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = transmute(&s as &str);
        forget(s);
        ret
    }
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