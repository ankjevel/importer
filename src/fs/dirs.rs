use std::str;
use std::i64;
use std::os::unix::fs::MetadataExt;
use std::string::String;
use std::path::Path;
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

pub fn current_directory() -> String {
    current().unwrap().into_os_string().into_string().unwrap() + "/"
}

pub fn home_directory() -> String {
    home().unwrap().into_os_string().into_string().unwrap() + "/"
}

pub fn unwrap_created_date<'a>(path: &'a Path) -> String {
    let meta = &path.metadata().unwrap();
    let created: i64 = meta.ctime();

    created.to_string()
}

pub fn get_extension<'a>(path: &'a Path) -> &'static str {
    let extension = match path.extension() {
        None => "none",
        Some(ext) => ext.to_str().unwrap()
    };

    borrowed_string_to_static_str(&&*extension.to_lowercase())
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
