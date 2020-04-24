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
mod listener;

use listener::watch;

fn main() {
    if let Err(err) = watch() {
        panic!("{:?}", err)
    }
}
