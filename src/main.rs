#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

extern crate crypto;
extern crate dirs;
extern crate notify;
extern crate rusqlite;
extern crate serde;

const NTHREADS: i32 = 5;

mod cols;
mod config;
mod fs;

use cols::to_sep_col;
use fs::files::Files;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{sync::mpsc::channel, thread::spawn, time::Duration};

use self::config::CONFIG;

fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

    watcher
        .watch(&CONFIG.path.watch, RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error {}", e),
        }
    }
}

fn check(path: String) {
    let (tx, rx) = channel();
    let mut files = Files::new();

    files.check(&path);

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
    check(CONFIG.path.images.to_string());
    if let Err(err) = watch() {
        println!("Error! {:?}", err)
    }
}
