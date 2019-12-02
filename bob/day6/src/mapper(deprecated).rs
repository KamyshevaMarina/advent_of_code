use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Status {
    External,
    Internal,
}

#[derive(Debug)]
struct Location {
    area: u32,
    status: Status,
}

impl Location {
    fn new() -> Self {
        Location {
            area: 0,
            status: Status::Internal,
        }
    }
}

pub fn map_make(data: &str) {
    let mut locations = data.split('\n').collect::<Vec<&str>>();
    locations.pop().unwrap(); // popping the last empty line
    let locations = locations
        .iter()
        .map(|l| {
            l.split(',')
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|v| (v[0], v[1]))
        .collect::<Vec<(u32, u32)>>();
    let &(x_max, _) = locations.iter().max_by(|(x, _), (y, _)| x.cmp(y)).unwrap();
    let &(_, y_max) = locations.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

    let mut locations = locations
        .iter()
        .map(|&p| (p, Location::new()))
        .collect::<HashMap<(u32, u32), Location>>();

    for i in 0..=x_max {
        for j in 0..=y_max {
            let mut external = false;
            if i == 0 || i == x_max || j == 0 || j == y_max {
                external = true;
            }
            let mut distance = u32::max_value();
            let mut disputed = false;
            let mut owner = (0, 0);
            for (&k, _) in &locations {
                let d = ((i as i32 - k.0 as i32).abs() + (j as i32 - k.1 as i32).abs()) as u32;
                if distance > d {
                    owner = k;
                    distance = d;
                    disputed = false;
                } else if distance == d {
                    disputed = true;
                }
            }
            if !disputed {
                let l = locations.get_mut(&owner).unwrap();
                l.area += 1;
                if external {
                    l.status = Status::External;
                }
            }
        }
    }
    let (k, v) = locations
        .iter()
        .filter(|(_, i)| i.status != Status::External)
        .max_by(|(_, i), (_, j)| i.area.cmp(&j.area))
        .unwrap();
    println!("COORDS: {:?}, AREA: {}", k, v.area);
}
