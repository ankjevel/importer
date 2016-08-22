use crypto::md5::Md5;
use crypto::digest::Digest;

use std::str;
use std::error::Error;
use std::io::prelude::*;
use std::string::String;
use std::fmt;
use std::path::Path;
use std::fs::File as fsFile;

use string::borrowed_string_to_static_str;
use super::dirs::{get_extension, unwrap_created_date};

fn generate_md5(bytes: &[u8]) -> String {
    let data = String::from_utf8_lossy(&bytes);

    let mut hasher = Md5::new();
    hasher.input_str(&data);

    let mut output = [0; 16];
    hasher.result(&mut output);
    hasher.result_str()
}

pub struct File {
    pub path_string: String,
    md5: String,
    extension: String,
    created: String,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "({:?}, {}, {}, {})",
               self.path_string,
               self.md5,
               self.created,
               self.extension)
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
            extension: String::from(borrowed_string_to_static_str(&extension)),
        }
    }

    pub fn set_md5(&mut self) {
        if self.md5 != "" {
            return;
        }

        let path = Path::new(&self.path_string);

        let display = path.display();
        let mut file = match fsFile::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        let mut bytes = Vec::new();
        match file.read_to_end(&mut bytes) {
            Err(why) => panic!("can't read file {}: {}", display, why.description()),
            Ok(_) => (),
        }

        self.md5 = generate_md5(&bytes);
    }
}
