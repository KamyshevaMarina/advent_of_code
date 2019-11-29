pub mod utils {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    enum Action {
        Shift, // started the shift
        Sleep, // fell asleep
        Wake,  // woke up
    }

    #[derive(Debug)]
    pub struct Timestamp {
        action: Action,
        minute: u32,
        guard_id: u32,
    }

    #[derive(Debug)]
    pub struct Guard {
        id: u32,
        minutes_total: u32,
        state: Action,
        last_minute: u32,
    }

    impl Timestamp {
        fn new(timestamp: &str, id: u32) -> Self {
            Timestamp {
                minute: timestamp[15..17].parse::<u32>().unwrap(),
                action: match timestamp.find(|c: char| match c {
                    '#' => true,
                    'w' => true,
                    'l' => true,
                    _ => false,
                }) {
                    Some(19) => Action::Wake,
                    Some(21) => Action::Sleep,
                    Some(25) => Action::Shift,
                    Some(_) => panic!("Something is wrong with input"),
                    None => panic!("Something is wrong with input"),
                },
                guard_id: id,
            }
        }
    }

    fn parse(data: &str) -> Vec<&str> {
        let mut lines = data.split('\n').collect::<Vec<&str>>();
        lines.pop().unwrap();
        lines.sort_unstable();
        lines
    }

    fn extract_id(action: &str) -> u32 {
        let iter = action.chars().skip(26);
        let mut end: usize = 0;
        for (i, c) in iter.enumerate() {
            if c.is_whitespace() {
                end = i;
                break;
            }
        }
        action[26..26 + end].parse().unwrap()
    }

    fn structure_timeline(timeline: Vec<&str>) -> Vec<Timestamp> {
        let mut current_id: u32 = 0;
        let timeline: Vec<Timestamp> = timeline
            .iter()
            .map(|i| {
                if let Some(_) = i.find('#') {
                    current_id = extract_id(i);
                }
                Timestamp::new(i, current_id)
            })
            .collect();
        timeline
    }

    pub fn get_target(data: &str) -> u32 {
        let timeline = parse(data);
        let timeline = structure_timeline(timeline);
        let mut guards: HashMap<u32, Guard> = HashMap::new();
        for t in &timeline {
            if let Some(g) = guards.get_mut(&t.guard_id) {
                match t.action {
                    Action::Wake => {
                        g.minutes_total += t.minute - g.last_minute;
                        g.last_minute = 0;
                    }
                    Action::Sleep => {
                        g.state = t.action.clone();
                        g.last_minute = t.minute;
                    }
                    _ => {
                        g.state = t.action.clone();
                        g.last_minute = 0;
                    }
                }
            } else {
                guards.insert(
                    t.guard_id,
                    Guard {
                        id: t.guard_id,
                        minutes_total: 0,
                        last_minute: 0,
                        state: t.action.clone(),
                    },
                );
            }
        }
        let guards: Vec<Guard> = guards.drain().map(|(_, v)| v).collect();
        let target = guards
            .iter()
            .max_by(|x, y| x.minutes_total.cmp(&y.minutes_total))
            .unwrap();

        let target_timeline: Vec<Timestamp> = timeline
            .into_iter()
            .filter(|i| i.guard_id == target.id)
            .collect();
        target_timeline.iter().for_each(|i| println!("{:?}", i));

        let mut iter = target_timeline.iter();
        let mut minutes: Vec<Vec<u32>> = Vec::new();
        loop {
            match iter.next() {
                Some(t) => match t.action {
                    Action::Sleep => {
                        minutes.push((t.minute..iter.next().unwrap().minute).collect());
                    }
                    _ => continue,
                },
                None => break,
            }
        }
        let minutes: Vec<u32> = minutes.into_iter().flatten().collect();
        let mut counts: HashMap<u32, u32> = HashMap::new();
        minutes
            .iter()
            .for_each(|&i| *counts.entry(i).or_insert(0) += 1);
        let (k, _) = counts.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
        println!("{}", k);
        counts.iter().for_each(|i| println!("{:?}", i));
        target.id * k
    }
}
