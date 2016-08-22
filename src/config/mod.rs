extern crate toml;

use toml::{Parser, Value};

use std::str;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use std::path::Path;
use std::env::var;
use std::collections::BTreeMap;

use fs::dirs::unwrap_path;

use string::{string_to_static_str, borrowed_string_to_static_str};

fn get_config_path() -> &'static str {
    match var("CONFIG") {
        Ok(val) => borrowed_string_to_static_str(&val),
        Err(_) => "Config.toml",
    }
}

fn read_config<'a>(path: &'a Path) -> Parser<'a> {
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let extension = match path.extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap(),
    };

    if extension.to_lowercase() != "toml" {
        panic!("not a config file, {}: {}", display, extension);
    }

    let mut bytes = Vec::new();
    match file.read_to_end(&mut bytes) {
        Err(why) => panic!("can't read file {}: {}", display, why.description()),
        Ok(_) => (),
    }
    Parser::new(&string_to_static_str(String::from_utf8_lossy(&bytes).into_owned()))
}

pub struct Config {
    parser: BTreeMap<String, Value>,
}

impl Config {
    pub fn new() -> Config {
        Config { parser: match read_config(&unwrap_path(&get_config_path())).parse() {
            Some(cfg) => cfg,
            None => panic!("can't unwrap config"),
        } }
    }

    fn query(&mut self, table: &'static str, q: &'static str) -> Value {
        match self.parser.get(table) {
            Some(table) => table.lookup(q).unwrap().clone(),
            None => panic!("table does not exist"),
        }
    }

    pub fn query_str(&mut self, table: &'static str, q: &'static str) -> &'static str {
        borrowed_string_to_static_str(&self.query(table, q).as_str().unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use config;
}
