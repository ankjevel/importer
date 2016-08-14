use std::str;
use std::error::Error;
use std::io::prelude::*;
use std::fs::{File};
use std::string::String;
use std::path::Path;

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