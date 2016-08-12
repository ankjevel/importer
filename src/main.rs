extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

mod file;
mod hash;

fn main() {
    {
        let files = file::Files::new();
        files.check(".");
        // loop {
        //     match files.next() {
        //         Some(v) => { println!("{:?}", v); }
        //         None => {break;}
        //     }
        // }
    }

    println!("{}", hash::md5::from_str("key"));
}
