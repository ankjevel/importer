extern crate toml;

use toml::{
    Parser,
    Value
};

use std::str;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use std::path::Path;
use std::env::var;

use file::unwrap_path;
use string::{
    string_to_static_str,
    borrowed_string_to_static_str
};

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
        Ok(file) => file
    };

    let extension = match path.extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap()
    };

    if extension.to_lowercase() != "toml" {
        panic!("not a config file, {}: {}", display, extension);
    }

    let mut bytes = Vec::new();
    match file.read_to_end(&mut bytes) {
        Err(why) => panic!("can't read file {}: {}", display, why.description()),
        Ok(_) => ()
    }
    let contents = string_to_static_str(String::from_utf8_lossy(&bytes).into_owned());

    Parser::new(&contents)
}

pub struct Config<'a> {
    parser: Parser<'a>
}

impl<'a> Config<'a> {
    pub fn new<'life>() -> Config<'life> {
        let unwraped_path = unwrap_path(&get_config_path());
        let file: &'life Path = unwraped_path;

        Config {
            parser: read_config(&file)
        }
    }

    pub fn query(&mut self, table: &'static str, q: &'static str) -> &'static str {
        println!("q: {:?}", q);
        let values = match self.parser.parse() {
            Some(values) => values,
            None => panic!("unwrap failed")
        };

        let table: &Value = match values.get(table) {
            Some(table) => table,
            None => panic!("table does not exist")
        };

        println!("values: '{:?}', table: '{:?}'", &values, &table);

        let lookup = &table.lookup(q).unwrap().as_str().unwrap();

        borrowed_string_to_static_str(&lookup)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use config;
}
