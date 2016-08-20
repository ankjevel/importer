extern crate crypto;
extern crate toml;

pub mod file;
pub mod config;
pub mod string;

use std::env::var;

fn get_config_path() -> &'static str {
    let default_value: &'static str = "~/Config.toml";
    match var("CONFIG") {
        Ok(val) => string::string_to_static_str(val),
        Err(_) => default_value,
    }
}

fn main() {
    let mut conf = config::Config::new(get_config_path());
    let path = &conf.query("paths", "images");

    let mut files = file::Files::new();
    files.check(path);

    for (file_path, md5) in files.md5s {
        println!("path: {}, md5: {}", file_path, md5)
    }
}
