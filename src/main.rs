extern crate crypto;
extern crate toml;

mod file;
mod config;
mod string;

fn main() {
    let mut conf = config::Config::new("~/Config.toml");
    let path = &conf.query("paths", "images");

    let mut files = file::Files::new();

    files.check(path);

    for md5 in files.md5s {
        println!("md5: {}", md5)
    }
}