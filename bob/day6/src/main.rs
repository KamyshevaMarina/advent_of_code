extern crate day6;
use day6::mapper;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data.txt").expect("failed to open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("failed to read from file");
    mapper::map_make(&s);
}
