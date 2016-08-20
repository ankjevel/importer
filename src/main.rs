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

    for mut file in files.collection {
        println!("fn: {}", &file.path_string);
        file.set_md5();
        file.set_created();
        println!("file: {}", &file)
    }
}
