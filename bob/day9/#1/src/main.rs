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
    let mut players = [0; 410];
    for marble in 1..7205901 {
        players[(marble - 1) % 410] += circle.insert(marble);
        // print!(
        //     "{:?} ",
        //     circle
        //         .current
        //         .borrow()
        //         .previous
        //         .clone()
        //         .unwrap()
        //         .borrow()
        //         .value
        // );
        // print!("{} ", circle.current.borrow().value);
        // print!(
        //     "{:?}\n",
        //     circle.current.borrow().next.clone().unwrap().borrow().value
        // );
    }
    println!(
        "Winner: {:?}",
        players
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap()
    );
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
    fn insert(&mut self, value: usize) -> usize {
        if value % 23 == 0 {
            return value + self.remove();
        }
        let previous = &mut self.current.borrow().next.clone().unwrap();
        let next = previous.borrow().next.clone().unwrap();
        let new = Rc::new(RefCell::new(Node {
            next: Some(next.clone()),
            previous: Some(previous.clone()),
            value,
        }));
        previous.borrow_mut().next = Some(new.clone());
        next.borrow_mut().previous = Some(new.clone());
        self.current = new;
        0
    }
    fn remove(&mut self) -> usize {
        for _ in 0..6 {
            let t = self.current.borrow().previous.clone();
            self.current = t.unwrap();
        }
        let previous = mem::replace(&mut self.current.borrow_mut().previous, None).unwrap();
        let pp = mem::replace(&mut previous.borrow_mut().previous, None).unwrap();
        pp.borrow_mut().next = Some(self.current.clone());
        self.current.borrow_mut().previous = Some(pp);
        let value = previous.borrow().value;
        value
    }
}
