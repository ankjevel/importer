use crate::{cols::to_sep_col, config::CONFIG, fs::files::Files, NTHREADS};
use notify::{
    DebouncedEvent::{Create, Remove},
    RecommendedWatcher, RecursiveMode, Watcher,
};
use std::{path::PathBuf, sync::mpsc::channel, thread::spawn, time::Duration};

fn path_to_string(path: &PathBuf) -> String {
    path.to_str().unwrap_or("").to_string()
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

pub fn watch() -> notify::Result<()> {
    check(CONFIG.path.images.to_string());

    let (tx, rx) = channel();
    let device = (CONFIG.path.watch.to_string() + &CONFIG.path.device).to_string();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

    watcher
        .watch(&CONFIG.path.watch, RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                Remove(path) => {
                    if path_to_string(&path).contains(&device) {
                        println!("Remove: {:?}", path)
                    }
                }
                Create(path) => {
                    if path_to_string(&path).contains(&device) {
                        println!("Create: {:?}", path)
                    }
                }
                _ => continue,
            },
            Err(e) => panic!("watch error {}", e),
        }
    }
}
