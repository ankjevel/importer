use std::str;
use std::error::Error;
use std::io::prelude::*;
use std::fs::{File};
use std::string::String;
use std::path::Path;
use file::unwrap_path;

fn read_config(path: &Path) -> String {
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
  String::from_utf8_lossy(&bytes).into_owned()
}

pub struct Config {}

impl Config {
  pub fn new<'life>(file_path: &'life str) -> Config {
    let conf = Config {};

    let unwraped_path = unwrap_path(&&*file_path);
    let file = Path::new(&unwraped_path);
    // println!("dir? {}", PathBuf::from("~").to_str().unwrap());
    // println!("current {}", current_dir().unwrap().display());
    read_config(&file);

    conf
  }
}

