use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let tpl = fs::read_to_string("data.txt").unwrap();
    let mut tpl = tpl
        .split('\n')
        .map(|i| i.as_bytes())
        .collect::<Vec<&[u8]>>();
    tpl.pop();

    let mut ts = HashSet::new();
    let mut ps = HashSet::new();

    let tpl = tpl.iter().map(|i| (i[36], i[5])).collect::<Vec<(u8, u8)>>();

    tpl.iter().for_each(|(t, p)| {
        ts.insert(*t);
        ps.insert(*p);
    });

    let mut queue = ps.difference(&ts).map(|&i| i).collect::<Vec<u8>>();

    let mut targets: HashMap<u8, HashSet<u8>> = HashMap::new();

    tpl.iter().for_each(|(t, p)| {
        targets.entry(*t).or_insert(HashSet::new()).insert(*p);
    });

    queue.sort(); // sort in alphabetical order
    queue.reverse(); // reverse so we can use pop()

    let mut result: Vec<u8> = Vec::new();

    while let Some(job) = queue.pop() {
        targets.iter_mut().for_each(|(_, v)| {
            v.remove(&job);
        });
        targets.iter().for_each(|(k, v)| {
            if v.is_empty() {
                queue.push(*k);
            }
        });

        for j in queue.iter() {
            targets.remove(j);
        }

        result.push(job);
        queue.sort();
        queue.reverse();
    }
    println!("THE RESULT IS: {:?}", result);
    println!("THE RESULT IS: {:?}", String::from_utf8(result));
}
