use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

fn main() {
    let mut circle = Circle {
        current: Rc::new(RefCell::new(Node {
            next: None,
            previous: None,
            value: 0,
        })),
    };
    circle.current.borrow_mut().next = Some(circle.current.clone());
    circle.current.borrow_mut().previous = Some(circle.current.clone());

    circle.insert(832);
    circle.insert(45);
    println!("{:?}", circle.current.borrow_mut().value);
}

#[derive(Debug)]
struct Circle {
    current: Rc<RefCell<Node>>,
}

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
    previous: Option<Rc<RefCell<Node>>>,
    value: usize,
}

impl Circle {
    fn insert(&mut self, value: usize) {
        let next = mem::replace(&mut self.current.borrow_mut().next, None).unwrap();
        let new = Rc::new(RefCell::new(Node {
            next: Some(next.clone()),
            previous: Some(self.current.clone()),
            value,
        }));
        self.current.borrow_mut().next = Some(new.clone());
        next.borrow_mut().previous = Some(new.clone());
        self.current = new;
    }
}
