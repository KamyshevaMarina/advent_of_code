use std::collections::HashSet;

fn main() {
    let mut s = std::fs::read_to_string("data.txt").unwrap();
    s.pop();
    let mut s = s.split('\n').map(|i| i.as_bytes()).collect::<Vec<&[u8]>>();

    let mut planted: HashSet<&[u8]> = HashSet::new();
    while let Some(l) = s.pop() {
        if l.is_empty() {
            break;
        }
        if let 0x23 = &l[9] {
            planted.insert(&l[0..5]);
        }
    }
    let mut state = vec![0x2E_u8; 30];
    state.append(&mut (&s.remove(0)[15..]).to_vec());
    state.append(&mut vec![0x2E_u8; 30]);
    println!("{:2}: {}", 0, String::from_utf8(state.clone()).unwrap());
    for i in 1..201 {
        let mut nextgen = vec![0x2E_u8; 2];
        for w in state.windows(5) {
            if planted.contains(w) {
                nextgen.push(0x23);
            } else {
                nextgen.push(0x2E);
            }
        }
        nextgen.append(&mut vec![0x2E_u8; 2]);
        state = nextgen;
        println!("{:2}: {}", i, String::from_utf8(state.clone()).unwrap());
    }
    println!(
        "ANSWER: {}",
        state.iter().enumerate().fold(0, |acc, i| if *i.1 == 0x23 {
            println!("hit: {}", i.0);
            acc + i.0 - 30
        } else {
            acc
        })
    );
}
