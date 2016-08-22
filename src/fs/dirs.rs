use std::str;
use std::i64;
use std::os::unix::fs::MetadataExt;
use std::string::String;
use std::path::Path;
use std::env::{current_dir as current, home_dir as home};

use string::{string_to_static_str, borrowed_string_to_static_str};

const SLASH: u8 = '/' as u8;
const TIDLE: u8 = '~' as u8;

macro_rules! expand {
    ( $( &$x: expr ),* ) => {
        {
            let mut string = String::new();
            $(
                string.push_str($x.unwrap().to_str().unwrap());
            )*
            string.push_str(&"/");
            string
        }
    };
}

fn current_directory() -> String {
    expand!(&current())
}

fn home_directory() -> String {
    expand!(&home())
}

pub fn unwrap_created_date<'a>(path: &'a Path) -> String {
    let meta = &path.metadata().unwrap();
    let created: i64 = meta.ctime();

    created.to_string()
}

pub fn get_extension<'a>(path: &'a Path) -> &'static str {
    let extension = match path.extension() {
        None => "none",
        Some(ext) => ext.to_str().unwrap(),
    };

    borrowed_string_to_static_str(&&*extension.to_lowercase())
}

pub fn unwrap_path<'a>(file_path: &str) -> &'a Path {
    let first_byte = file_path.chars().nth(0).unwrap() as u8;
    let mut file_path_copy = String::new();
    file_path_copy.push_str(&file_path);
    let mut mutable_str = String::new();
    match first_byte {
        SLASH => (),
        TIDLE => {
            mutable_str.push_str(&home_directory());
            &file_path_copy.remove(0);
            match file_path_copy.chars().nth(0).unwrap() as u8 {
                SLASH => {
                    &file_path_copy.remove(0);
                }
                _ => (),
            }
        }
        _ => {
            mutable_str.push_str(&current_directory());
        }
    }
    mutable_str.push_str(&file_path_copy);
    let s: &'static str = string_to_static_str(mutable_str);
    Path::new(s)
}
