use std::fs::File;
use std::io::prelude::*;

pub fn read_data(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut data = String::new();
    f.read_to_string(&mut data).expect("read file failed");
    data
}
