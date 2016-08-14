extern crate crypto;
extern crate toml;

mod file;
mod config;

fn main() {
    config::Config::new("~/Config.toml");
    let mut files = file::Files::new();
    files.check("/Users/dpn/dev/python/import-old/temp");

    for md5 in files.md5s {
        println!("md5: {}", md5)
    }
}