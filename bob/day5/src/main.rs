extern crate day5;
use day5::challenge1::polymer;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data.txt").expect("FAILED TO READ THE FILE");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("FAILED TO READ FROM FILE");
    let s = s.trim();
    let mut result: Vec<(u8, usize)> = Vec::new();
    for (u, l) in (b'A'..=b'Z').zip(b'a'..=b'z') {
        let replaced = s.replace(|c: char| c as u8 == u || c as u8 == l, "");
        let mut reader = polymer::Reader::new();
        replaced.as_bytes().iter().for_each(|&i| reader.try_read(i));
        result.push((u, reader.len()));
    }
    result
        .iter()
        .for_each(|(c, len)| println!("{}: {}", *c as char, len));
    let (c, len) = result.iter().min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
    println!("THE WINNER: {} - {}", c, len);
}
