use std::fs::File;
use std::io::Read;

struct Patch {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
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
            .collect::<Vec<usize>>();
        patches.push(Patch {
            x: t[1],
            y: t[2],
            width: t[3],
            height: t[4],
        })
    }
    patches
}

fn walk() {
    let mut count = 0;
    let mut f = File::open("data.txt").expect("filesystem error");
    let mut s: String = String::new();
    f.read_to_string(&mut s).expect("io error");
    let patches = parse(&s);
    let mut inches = vec![vec![0; 1000]; 1000];
    for p in patches {
        for y in p.y + 1..p.y + p.height + 1 {
            for x in p.x + 1..p.x + p.width + 1 {
                inches[y][x] += 1;
                //if inches[y][x] != 2 {
                //    if inches[y][x] >= 2 {
                //        inches[y][x] = 2;
                //        count += 1;
                //    }
                //}
            }
        }
    }
    count = inches.into_iter().flatten().filter(|i| i > &1).count();
    println!("count {}", count);
}
