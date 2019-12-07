use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

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
    let mut queuer = Queuer::new(5, ptl);
    let result = queuer.run();

    println!("THE RESULT IS: {:?}", result);
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

struct Queuer {
    workers: Vec<Worker>,
    result: Rc<RefCell<Vec<u8>>>,
    rsq: Rc<RefCell<Vec<u8>>>,
    tpsl: HashMap<u8, Vec<u8>>,
}

#[derive(Debug)]
struct Worker {
    job: u8,
    timer: u8,
    status: Status,
    result: Rc<RefCell<Vec<u8>>>,
    rsq: Rc<RefCell<Vec<u8>>>,
}

impl Worker {
    fn new(result: Rc<RefCell<Vec<u8>>>, rsq: Rc<RefCell<Vec<u8>>>) -> Self {
        Worker {
            job: 0,
            timer: 0,
            status: Status::Idle,
            result: result,
            rsq: rsq,
        }
    }

    fn dec(&mut self) -> Status {
        match self.status {
            Status::Busy => {
                if self.timer == self.job - 4 {
                    self.result.borrow_mut().push(self.job);
                    self.status = Status::Idle;
                    Status::Done(self.job)
                } else {
                    self.timer += 1;
                    Status::Busy
                }
            }
            _ => self.take_job(),
        }
    }

    fn take_job(&mut self) -> Status {
        if let Some(job) = self.rsq.borrow_mut().pop() {
            self.job = job;
            self.timer = 0;
            self.status = Status::Busy;
            Status::Busy
        } else {
            Status::Idle
        }
    }
}

impl Queuer {
    fn new(worker_num: usize, ptl: Vec<(u8, u8)>) -> Self {
        let result = Rc::new(RefCell::new(Vec::new()));
        let rsq = Rc::new(RefCell::new(Vec::new()));

        let mut tpsl: HashMap<u8, Vec<u8>> = HashMap::with_capacity(70); // target - prerequisites list
        ptl.iter()
            .for_each(|(p, t)| tpsl.entry(*t).or_insert(Vec::new()).push(*p));
        ptl.iter().for_each(|(p, _)| {
            if !tpsl.contains_key(p) {
                rsq.borrow_mut().push(*p)
            }
        });

        rsq.borrow_mut().sort(); // sort in alphabetical order
        rsq.borrow_mut().dedup(); // deduplicate
        rsq.borrow_mut().reverse(); // reverse so we can use pop()

        let mut workers = Vec::new();
        for _ in 0..worker_num {
            workers.push(Worker::new(result.clone(), rsq.clone()));
        }

        Queuer {
            workers: workers,
            result: result,
            rsq: rsq,
            tpsl: tpsl,
        }
    }

    fn run(&mut self) -> usize {
        let mut seconds: usize = 0;
        let total = self.tpsl.len() + self.rsq.borrow().len();
        for w in &mut self.workers {
            w.dec();
        }
        let mut tpsl = self.tpsl.clone();
        loop {
            for w in &mut self.workers {
                if let Status::Done(job) = w.dec() {
                    tpsl.iter_mut().for_each(|(_, v)| {
                        if v.contains(&job) {
                            remove_item(v, job)
                        }
                    });

                    let mut t: Vec<u8> = tpsl
                        .iter()
                        .filter(|(_, v)| v.len() == 0)
                        .map(|(k, _)| *k)
                        .collect();

                    t.iter().for_each(|i| {
                        tpsl.remove(i);
                    });

                    self.rsq.borrow_mut().append(&mut t);
                    self.rsq.borrow_mut().sort();
                    self.rsq.borrow_mut().reverse();
                    w.take_job();
                }
            }
            if total == self.result.borrow().len() {
                break;
            }

            seconds += 1;
            println!("{}: {:?}", seconds, tpsl);
        }
        seconds
    }
}

#[derive(Debug)]
enum Status {
    Busy,
    Idle,
    Done(u8),
}
