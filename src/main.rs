extern crate crypto;
extern crate toml;

mod fs;
mod config;
mod string;

use std::thread;
use std::sync::mpsc;

static NTHREADS: f32 = 15.0;

fn to_sep_col<'a >(col: &mut Vec<fs::file::File>) -> Vec<Vec<fs::file::File>>{
    let x = col.len() as f32 / NTHREADS;

    let extra = ((x % 1.0) * NTHREADS) as i32;

    let iterations = x as i32;
    let mut vec = vec![iterations; NTHREADS as usize];

    for i in 0..extra {
        vec[i as usize] = iterations + 1
    }

    let mut sliced: Vec<Vec<fs::file::File>> = Vec::new();
    let mut curr = 0;
    for i in 0..(NTHREADS as i32) {
        let iter = vec[i as usize];
        let mut inner_vec: Vec<fs::file::File> = Vec::new();
        for _ in 0..iter {
            let file: fs::file::File = col.pop().unwrap();
            inner_vec.push(file);
            curr = curr + 1;
        }
        sliced.push(inner_vec);
    }


    sliced
}

fn main() {
    let mut conf = config::Config::new();
    let mut files = fs::files::Files::new();
    files.check(conf.query_str("paths", "images"));

    let (tx, rx) = mpsc::channel();

    for vec in to_sep_col(files.collection_mut()) {
        let transmitter = tx.clone();
        thread::spawn(move || {
            let mut results = Vec::new();
            for mut file in vec {
                &file.set_md5();
                results.push(file);
            }
            transmitter.send(results).unwrap();
        });
    }

    for _ in 0..(NTHREADS as i32) {
        for file in rx.recv().unwrap() {
            println!("file! {}", file);
        }
    }
}
