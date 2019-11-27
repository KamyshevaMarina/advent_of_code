use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data.txt").expect("filesystem error");
    let mut s: String = String::new();
    f.read_to_string(&mut s).expect("io error");
    let mut numbers = s
        .split('\n')
        .map(|i| i.parse().unwrap_or(0))
        .collect::<Vec<i32>>();
    numbers.pop().unwrap();

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
