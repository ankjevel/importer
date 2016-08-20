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

    for (file_path, md5) in files.md5s {
        println!("path: {}, md5: {}", file_path, md5)
    }
}

