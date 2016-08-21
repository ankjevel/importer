extern crate crypto;
extern crate toml;

mod fs;
mod config;
mod string;

use std::thread::spawn;
use std::sync::mpsc::channel;

use fs::file::File;
use fs::files::Files;
use config::Config;

static NTHREADS: i32 = 15;

fn to_sep_col<'a>(col: &mut Vec<File>) -> Vec<Vec<File>> {
    let x = col.len() as f32 / (NTHREADS as f32);
    let (extra, iterations) = (((x % 1.0) * (NTHREADS as f32)) as i32, x as i32);
    let mut vec = vec![iterations; NTHREADS as usize];

    for i in 0..extra {
        vec[i as usize] = iterations + 1
    }

    let mut sliced: Vec<Vec<File>> = Vec::new();
    let mut curr = 0;
    for i in 0..NTHREADS {
        let iter = vec[i as usize];
        let mut inner_vec: Vec<File> = Vec::new();
        for _ in 0..iter {
            let file: File = col.pop().unwrap();
            inner_vec.push(file);
            curr = curr + 1;
        }
        sliced.push(inner_vec);
    }


    sliced
}

fn main() {
    let (mut conf, mut files) = (Config::new(), Files::new());
    files.check(conf.query_str("paths", "images"));

    let (tx, rx) = channel();

    for vec in to_sep_col(files.collection_mut()) {
        let transmitter = tx.clone();
        spawn(move || {
            let mut results = Vec::new();
            for mut file in vec {
                &file.set_md5();
                results.push(file);
            }
            transmitter.send(results).unwrap();
        });
    }

    for _ in 0..NTHREADS {
        for file in rx.recv().unwrap() {
            println!("file! {}", file);
        }
    }
}
