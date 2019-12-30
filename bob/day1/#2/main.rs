use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut s = std::fs::read_to_string("data.txt").unwrap();
    s.pop();
    let numbers = s.split('\n').map(|i| i.parse::<i32>().unwrap());
    let mut acc: i32 = 0;
    let mut set: HashSet<i32> = HashSet::new();
    for n in numbers.iter().cycle() {
        acc += n;
        if !set.insert(acc) {
            break;
        }
    }
    println!("repeated: {}", acc);
}
