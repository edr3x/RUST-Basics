use std::fs::{self, File};
use std::io;

// fn read_username_from_file() -> Result<String, io::Error> {
// let mut f = File::open("Hello.txt")?;
// let mut s = String::new();

// f.read_to_string(&mut s)?;
// Ok(s)
// }

fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("Hello.txt")
}

fn main() {}
