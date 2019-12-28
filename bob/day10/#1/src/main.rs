const SIZE: usize = 355; // spotlight count
const ACC: i32 = 10300; // acceleration factor

fn main() {
    let mut s = std::fs::read_to_string("data.txt").unwrap();
    s.pop();
    let mut spotlights = SpotLights::new(s.split('\n').collect::<Vec<&str>>());
    loop {
        spotlights.displace();
        if spotlights.delta() > 0 {
            spotlights.replace();
            spotlights.draw();
            break;
        }
    }
}

struct SpotLights {
    xp: Vec<i32>, // x position
    yp: Vec<i32>, // y position
    xv: Vec<i32>, // x velocity
    yv: Vec<i32>, // y velocity
    area: i64,
}

impl SpotLights {
    fn new(data: Vec<&str>) -> Self {
        let mut xp: Vec<i32> = Vec::with_capacity(SIZE);
        let mut yp: Vec<i32> = Vec::with_capacity(SIZE);
        let mut xv: Vec<i32> = Vec::with_capacity(SIZE);
        let mut yv: Vec<i32> = Vec::with_capacity(SIZE);
        for d in data {
            xv.push(d[36..38].trim_start().parse::<i32>().unwrap());
            yv.push(d[40..42].trim_start().parse::<i32>().unwrap());
            xp.push(d[10..16].trim_start().parse::<i32>().unwrap() + xv.last().unwrap() * ACC);
            yp.push(d[18..24].trim_start().parse::<i32>().unwrap() + yv.last().unwrap() * ACC);
        }
        SpotLights {
            xp,
            yp,
            xv,
            yv,
            area: i64::max_value(),
        }
    }

    fn displace(&mut self) {
        for i in 0..SIZE {
            self.xp[i] += self.xv[i];
            self.yp[i] += self.yv[i];
        }
    }

    fn replace(&mut self) {
        for i in 0..SIZE {
            self.xp[i] -= self.xv[i];
            self.yp[i] -= self.yv[i];
        }
    }

    fn delta(&mut self) -> i64 {
        let a = self.area;
        self.area = (self.xp.iter().max().unwrap() - self.xp.iter().min().unwrap() + 1).abs()
            as i64
            * (self.yp.iter().max().unwrap() - self.yp.iter().min().unwrap() + 1).abs() as i64;
        self.area - a
    }

    fn draw(&self) {
        let x = (self.xp.iter().max().unwrap() - self.xp.iter().min().unwrap() + 1).abs() as usize;
        let y = (self.yp.iter().max().unwrap() - self.yp.iter().min().unwrap() + 1).abs() as usize;
        let (dx, dy) = (self.xp.iter().min().unwrap(), self.yp.iter().min().unwrap());
        let mut canvas: Vec<Vec<u8>> = Vec::with_capacity(y);
        for i in 0..y {
            canvas.push(Vec::with_capacity(x));
            for _ in 0..x {
                canvas[i].push('.' as u8);
            }
        }
        for i in 0..SIZE {
            let x = (self.xp[i] - dx) as usize;
            let y = (self.yp[i] - dy) as usize;
            canvas[y][x] = '#' as u8;
        }
        for i in 0..y {
            println!("{}", String::from_utf8(canvas[i].clone()).unwrap());
        }
    }
}
