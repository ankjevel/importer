extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::str::Chars;

pub fn from_str(key: &str) -> String {
  let mut hasher = Md5::new();
  hasher.input_str(key);
  let mut output = [0; 16]; // An MD5 is 16 bytes
  hasher.result(&mut output);
  let result = hasher.result_str();

  result
}
