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
    let default_value: &'static str = "Config.toml";
    match var("CONFIG") {
        Ok(val) => borrowed_string_to_static_str(&val),
        Err(_) => default_value,
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
    let contents = string_to_static_str(String::from_utf8_lossy(&bytes).into_owned());

    Parser::new(&contents)
}

pub struct Config {
    parser: BTreeMap<String, Value>,
}

impl Config {
    pub fn new() -> Config {
        let unwraped_path = unwrap_path(&get_config_path());
        let cfg = match read_config(&unwraped_path).parse() {
            Some(cfg) => cfg,
            None => panic!("can't unwrap config"),
        };

        Config { parser: cfg }
    }

    fn query(&mut self, table: &'static str, q: &'static str) -> Value {
        let table: &Value = match self.parser.get(table) {
            Some(table) => table,
            None => panic!("table does not exist"),
        };

        table.lookup(q).unwrap().clone()
    }

    pub fn query_str(&mut self, table: &'static str, q: &'static str) -> &'static str {
        let result = self.query(table, q);

        borrowed_string_to_static_str(result.as_str().unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use config;
}
