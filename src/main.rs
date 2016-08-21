extern crate crypto;
extern crate toml;

mod fs;
mod config;
mod string;

// use std::thread;
// static NTHREADS: i32 = 10;

fn main() {
    let mut conf = config::Config::new();
    let mut files = fs::files::Files::new();
    files.check(conf.query_str("paths", "images"));

    // let mut col = &files.collection_mut();
    // println!("i: {}", col.len());
    // for i in 0..NTHREADS {
    //     children.push(thread::spawn(move || {
    //         println!("this is thread number {}", i)
    //     }));
    // }

    for file in files.collection_mut() {
        println!("fn: {}", &file.path_string);
        &file.set_md5();
        println!("file: {}", &file)
    }

    for file in files.collection_mut() {
        println!("file: {}", &file)
    }
}
