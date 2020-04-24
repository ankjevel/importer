use crypto::{digest::Digest, md5::Md5};

use std::{
    fmt::{Display, Formatter, Result},
    fs::File as fsFile,
    io::prelude::*,
    path::Path,
    string::String,
};

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

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "({:?}, {}, {}, {})",
            self.path_string, self.md5, self.created, self.extension
        )
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
            extension: extension.to_string(),
        }
    }

    pub fn set_md5(&mut self) {
        if self.md5 != "" {
            return;
        }

        let path = Path::new(&self.path_string);
        let display = path.display();

        let mut file = match fsFile::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut buffer = [0; 150];
        match file.read(&mut buffer) {
            Err(why) => panic!("can't read file {}: {}", display, why),
            Ok(_) => (),
        }

        self.md5 = generate_md5(&buffer);
    }
}
