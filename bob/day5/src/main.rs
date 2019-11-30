extern crate day5;
use day5::challenge1::polymer;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data.txt").expect("FAILED TO READ THE FILE");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("FAILED TO READ FROM FILE");
    let s = s.trim();
    let mut reader = polymer::Reader::new();
    s.as_bytes().iter().for_each(|&i| reader.try_read(i));
    reader.debug();
}
