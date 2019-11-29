pub mod utils {

    #[derive(Debug)]
    struct Patch {
        id: u32,
        x: u32, // upper left corner
        y: u32, // lower right corner
        width: u32,
        height: u32,
        disputed: bool,
    }

    impl Patch {
        #[inline(always)]
        fn intersect(&self, other: &Patch) -> bool {
            let max_x = if self.x > other.x { self } else { other };
            let max_y = if self.y > other.y { self } else { other };
            let min_x = if self.x < other.x { self } else { other };
            let min_y = if self.y < other.y { self } else { other };

            (min_x.x + min_x.width) > max_x.x && (min_y.y + min_y.height) > max_y.y
        }
    }

    fn parse(data: &str) -> Vec<Patch> {
        let mut patches: Vec<Patch> = Vec::new();
        let mut lines = data.split('\n').collect::<Vec<&str>>();
        lines.pop().unwrap();

        for l in lines.into_iter() {
            let t = l
                .split(|c: char| !c.is_digit(10))
                .filter(|i| i.len() > 0)
                .map(|i| i.parse().unwrap())
                .collect::<Vec<u32>>();
            if let [id, x, y, width, height] = t.as_slice() {
                patches.push(Patch {
                    id: *id,
                    x: *x,
                    y: *y,
                    width: *width,
                    height: *height,
                    disputed: false,
                });
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
                if inspected.intersect(other) {
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
