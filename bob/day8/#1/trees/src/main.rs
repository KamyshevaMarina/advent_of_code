use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

fn main() {
    let list = fs::read_to_string("data.txt").unwrap();
    let mut list = list.split('\n').collect::<Vec<&str>>();
    list.pop();
    let mut list = list[0]
        .split(' ')
        .filter_map(|i| i.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    list.reverse();
    let i = meta_get(Rc::new(RefCell::new(list)).clone());
    println!("RESULT: {}", i);
}

fn meta_get(list: Rc<RefCell<Vec<u32>>>) -> u32 {
    let count = list.borrow_mut().pop().unwrap();
    let meta_count = list.borrow_mut().pop().unwrap();
    let mut meta_sum = 0;
    if count > 0 {
        for _ in 0..count {
            meta_sum += meta_get(list.clone());
        }
    }
    for _ in 0..meta_count {
        meta_sum += list.borrow_mut().pop().unwrap();
    }
    meta_sum
}
