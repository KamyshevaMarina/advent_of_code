use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::rc::Rc;

const COUNT: usize = 5;

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

    let queue = Rc::new(RefCell::new(
        ps.difference(&ts).map(|&i| i).collect::<Vec<u8>>(),
    ));

    let result = Rc::new(RefCell::new(Vec::new()));

    let mut targets: HashMap<u8, HashSet<u8>> = HashMap::new();

    tpl.iter().for_each(|(t, p)| {
        targets.entry(*t).or_insert(HashSet::new()).insert(*p);
    });

    let targets = Rc::new(RefCell::new(targets));

    queue.borrow_mut().sort();
    queue.borrow_mut().reverse();

    let mut workers = Vec::new();

    for _ in 0..COUNT {
        workers.push(Worker {
            targets: targets.clone(),
            queue: queue.clone(),
            result: result.clone(),
            countdown: 0,
            job: 0,
        });
    }
    let mut seconds = 0;

    while result.borrow().len() != 26 {
        for w in &mut workers {
            w.tick();
        }
        for w in &mut workers {
            w.tack();
        }
        seconds += 1;
    }

    println!("ANSWER : {}", seconds - 1);
}

struct Worker {
    targets: Rc<RefCell<HashMap<u8, HashSet<u8>>>>,
    queue: Rc<RefCell<Vec<u8>>>,
    result: Rc<RefCell<Vec<u8>>>,
    countdown: u8,
    job: u8,
}

impl Worker {
    fn tick(&mut self) {
        if self.countdown == 0 && self.job != 0 {
            self.tock();
        } else if self.job == 0 {
            if let Some(job) = self.queue.borrow_mut().pop() {
                self.job = job;
                self.countdown = job - 5;
            }
        } else if self.countdown > 0 && self.job > 0 {
            self.countdown -= 1;
        }
    }
    fn tack(&mut self) {
        if self.job == 0 {
            if let Some(job) = self.queue.borrow_mut().pop() {
                self.job = job;
                self.countdown = job - 5;
            }
        }
    }

    fn tock(&mut self) {
        self.targets.borrow_mut().iter_mut().for_each(|(_, v)| {
            v.remove(&self.job);
        });
        self.targets.borrow_mut().iter().for_each(|(k, v)| {
            if v.is_empty() {
                self.queue.borrow_mut().push(*k);
            }
        });

        for j in self.queue.borrow().iter() {
            self.targets.borrow_mut().remove(j);
        }

        self.result.borrow_mut().push(self.job);
        self.job = 0;

        if self.queue.borrow().len() > 0 {
            self.queue.borrow_mut().sort();
            self.queue.borrow_mut().reverse();
        }
    }
}
