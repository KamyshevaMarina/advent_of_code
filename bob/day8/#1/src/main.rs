fn main() {
    let mut list = std::fs::read_to_string("data.txt").unwrap();
    // list.pop();
    let mut list = list
        .split(' ')
        .rev()
        .filter_map(|i| i.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    println!("{:?}", list);
    let i = meta_get(&mut list);
    println!("RESULT: {}", i);
}

fn meta_get(list: &mut Vec<u32>) -> u32 {
    let count = list.pop().unwrap();
    let meta_count = list.pop().unwrap();
    let mut meta_sum = 0;
    for _ in 0..count {
        meta_sum += meta_get(list);
    }
    for _ in 0..meta_count {
        meta_sum += list.pop().unwrap();
    }
    meta_sum
}
