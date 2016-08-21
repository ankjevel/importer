extern crate crypto;
extern crate toml;

mod file;
mod config;
mod string;

fn main() {
    let mut conf = config::Config::new();

    let mut files = file::Files::new();
    files.check(conf.query_str("paths", "images"));

    for file in files.collection_mut() {
        println!("fn: {}", &file.path_string);
        &file.set_md5();
        println!("file: {}", &file)
    }

    for file in files.collection_mut() {
        println!("file: {}", &file)
    }
}
