extern crate day4;
use day4::challenge1::utils;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data.txt").expect("couldn't open the file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("io error");
    let a = utils::get_target(&s);
    println!("{}", a);
}
