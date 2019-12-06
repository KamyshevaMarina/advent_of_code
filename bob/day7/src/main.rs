use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data.txt").expect("file not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("IO error");
    let mut ptl = s.split('\n').collect::<Vec<&str>>(); // prerequisite - target list
    ptl.pop().unwrap();
    let ptl: Vec<(u8, u8)> = ptl
        .iter()
        .map(|i| (i.as_bytes()[5], i.as_bytes()[36])) // extracting step designations
        .collect();
    let mut tpsl: HashMap<u8, Vec<u8>> = HashMap::with_capacity(70); // target - prerequisites list
    ptl.iter()
        .for_each(|(p, t)| tpsl.entry(*t).or_insert(Vec::new()).push(*p));
    let mut rsq: Vec<u8> = Vec::with_capacity(50); // reverse sorted queue
    ptl.iter().for_each(|(p, _)| {
        if !tpsl.contains_key(p) {
            rsq.push(*p)
        }
    });

    rsq.sort(); // sort in alphabetical order
    rsq.dedup();
    rsq.reverse(); // reverse so we can use pop()

    let mut result: Vec<u8> = Vec::new();

    while let Some(p) = rsq.pop() {
        tpsl.iter_mut().for_each(|(_, v)| {
            if v.contains(&p) {
                remove_item(v, p)
            }
        });
        let mut t: Vec<u8> = tpsl
            .iter()
            .filter(|(_, v)| v.len() == 0)
            .map(|(k, _)| *k)
            .collect();
        let clone = tpsl.clone();
        for (k, _) in clone.iter() {
            if t.contains(k) {
                tpsl.remove(k);
            }
        }
        rsq.append(&mut t);
        rsq.sort();
        rsq.reverse();
        result.push(p);
    }
    println!("THE RESULT IS: {:?}", result);
    println!("THE RESULT IS: {:?}", String::from_utf8(result));
}

fn remove_item<T>(vec: &mut Vec<T>, item: T)
where
    T: Eq + Copy,
{
    for (n, i) in vec.iter().enumerate() {
        if *i == item {
            vec.remove(n);
            return;
        }
    }
}
