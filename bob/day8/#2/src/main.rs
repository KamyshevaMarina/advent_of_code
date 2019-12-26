fn main() {
    let mut list = std::fs::read_to_string("data.txt").unwrap();
    list.pop();
    let mut list = list
        .split(' ')
        .filter_map(|i| i.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    list.reverse();
    let i = value_get(&mut list);
    println!("RESULT: {}", i);
}

fn value_get(list: &mut Vec<usize>) -> usize {
    let nodes = list.pop().unwrap();
    let meta = list.pop().unwrap();
    let mut value = 0;
    let mut children: Vec<usize> = Vec::new();
    for _ in 0..nodes {
        children.push(value_get(list));
    }
    for _ in 0..meta {
        let i = list.pop().unwrap();
        if nodes != 0 {
            if i <= nodes && i > 0 {
                value += children[i - 1];
            }
        } else {
            value += i;
        }
    }
    value
}
