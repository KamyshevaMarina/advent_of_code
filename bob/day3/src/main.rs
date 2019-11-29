extern crate day3;
extern crate time;
use day3::challenge2_2::utils::walk;
use std::fs::File;
use std::io::Read;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();
    let mut f = File::open("data.txt").expect("filesystem error");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("io error");
    println!("id: {}", walk(&s));
    let end = PreciseTime::now();
    println!("{} seconds for whatever you did.", start.to(end));
}
