extern crate crypto;
extern crate toml;

mod file;
mod config;
mod string;

fn main() {
    let mut conf = config::Config::new();
    let path = &conf.query("paths", "images");

    let mut files = file::Files::new();
    files.check(path);

    for file in files.collection.iter() {
        println!("file: {}", file)
    }
}

