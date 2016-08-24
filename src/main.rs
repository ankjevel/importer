extern crate crypto;
extern crate toml;
extern crate rusqlite;
extern crate notify;

static NTHREADS: i32 = 5;

mod fs;
mod config;
mod string;
mod cols;

use std::thread::spawn;
use std::sync::mpsc::channel;

use fs::files::Files;
use config::Config;
use cols::to_sep_col;

use notify::{RecommendedWatcher, Watcher, Event, Result};

fn watch() -> Result<()> {
    let (tx, rx) = channel();
    let mut conf = Config::new();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));

    try!(watcher.watch(conf.query_str("paths", "watch")));

    loop {
        match rx.recv() {
            Ok(Event {
                path: Some(path),
                op: Ok(op)
            }) => {
                println!("{:?} {:?}", op, path);
            },
            Err(e) => println!("watch error {}", e),
            _ => ()
        }
    }
}

fn check (path: &'static str) {
    let (tx, rx) = channel();
    let mut files = Files::new();

    files.check(path);

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

fn main() {
    let mut conf = Config::new();
    check(conf.query_str("paths", "images"));
    if let Err(err) = watch() {
        println!("Error! {:?}", err)
    }
}
