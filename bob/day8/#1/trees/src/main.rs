use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

fn main() {
    let mut list = fs::read_to_string("data.txt")
        .unwrap()
        .split('\n')
        .filter_map(|i| i.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    list.reverse();
    let list = Rc::new(RefCell::new(list));
    let i = meta_get(list.clone());
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
