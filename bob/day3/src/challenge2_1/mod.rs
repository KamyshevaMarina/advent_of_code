pub mod utils {
    use std::collections::HashSet;

    #[derive(Debug)]
    struct Patch {
        id: u32,
        disputed: bool,
        map: HashSet<(u32, u32)>,
    }

    impl Patch {
        fn new(id: u32) -> Self {
            Patch {
                id: id,
                disputed: false,
                map: HashSet::new(),
            }
        }
    }

    fn parse(data: &str) -> Vec<Patch> {
        let mut patches: Vec<Patch> = Vec::new();
        let mut lines = data.split('\n').collect::<Vec<&str>>();
        lines.pop().unwrap();

        for (i, l) in lines.into_iter().enumerate() {
            let t = l
                .split(|c: char| !c.is_digit(10))
                .filter(|i| i.len() > 0)
                .map(|i| i.parse().unwrap())
                .collect::<Vec<u32>>();
            if let [id, xc, yc, w, h] = t.as_slice() {
                patches.push(Patch::new(*id));
                for x in xc + 1..xc + w + 1 {
                    for y in yc + 1..yc + h + 1 {
                        patches[i].map.insert((x, y));
                    }
                }
            }
        }
        patches
    }

    pub fn walk(data: &str) -> u32 {
        let mut patches = parse(data);
        let lenght = patches.len();
        for i in 0..lenght - 1 {
            let mut iter = patches.iter_mut();
            let mut inspected = iter.nth(i).unwrap();
            for _ in i + 1..lenght {
                let mut other = iter.next().unwrap();
                if !(inspected.map.is_disjoint(&other.map)) {
                    inspected.disputed = true;
                    other.disputed = true;
                }
            }
            if !(inspected.disputed) {
                return inspected.id;
            }
        }
        0
    }
}
