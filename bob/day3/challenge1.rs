use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct Patch {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Hash, Eq, PartialEq)]
struct Inch {
    x: u32,
    y: u32,
}

fn parse(data: &str) -> Vec<Patch> {
    let mut patches: Vec<Patch> = Vec::new();
    let mut lines = data.split('\n').collect::<Vec<&str>>();
    lines.pop().unwrap();

    for l in lines {
        let t = l
            .split(|c: char| !c.is_digit(10))
            .filter(|i| i.len() > 0)
            .map(|i| i.parse().unwrap())
            .collect::<Vec<u32>>();
        patches.push(Patch {
            x: t[1],
            y: t[2],
            width: t[3],
            height: t[4],
        })
    }
    patches
}

fn main() {
    let mut f = File::open("data.txt").expect("filesystem error");
    let mut s: String = String::new();
    f.read_to_string(&mut s).expect("io error");
    let patches = parse(&s);
    let mut inches: HashMap<Inch, u32> = HashMap::new();
    for p in patches {
        for x in p.x + 1..p.x + p.width + 1 {
            for y in p.y + 1..p.y + p.height + 1 {
                let entry = inches.entry(Inch { x: x, y: y }).or_insert(0);
                *entry += 1;
            }
        }
    }
    println!(
        "Number of disputed inches is: {}",
        inches.iter().filter(|(_, &v)| v > 1).count()
    );
}
