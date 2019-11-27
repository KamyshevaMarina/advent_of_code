use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data.txt").expect("filesystem error");
    let mut s: String = String::new();
    f.read_to_string(&mut s).expect("io error");
    let mut ids = s.split('\n').collect::<Vec<&str>>();
    ids.pop().unwrap();

    let mut stop: bool = false;
    let mut wanted: String = String::new();

    for (i, x) in ids[..ids.len() - 1].iter().enumerate() {
        for y in &ids[i + 1..] {
            match compare(x, y) {
                (1, p) => {
                    stop = true;
                    wanted = x.to_string();
                    wanted.remove(p);
                }
                _ => continue,
            }
        }
        if stop {
            println!("The searched: {}", wanted);
            break;
        }
    }
}

fn compare(a: &str, b: &str) -> (usize, usize) {
    let mut diff: usize = 0;
    let mut pos: usize = 0;
    for (i, (x, y)) in a.chars().zip(b.chars()).enumerate() {
        if x != y {
            diff += 1;
            pos = i;
        }
    }
    (diff, pos)
}
