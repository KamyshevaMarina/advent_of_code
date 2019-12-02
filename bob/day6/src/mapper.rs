pub fn map_make(data: &str) {
    const TENK: i32 = 10_000;
    let mut locations = data.split('\n').collect::<Vec<&str>>();
    locations.pop().unwrap(); // popping the last empty line
    let locations = locations
        .iter()
        .map(|l| {
            l.split(',')
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| (v[0], v[1]))
        .collect::<Vec<(i32, i32)>>();
    let &(x_max, _) = locations.iter().max_by(|(x, _), (y, _)| x.cmp(y)).unwrap();
    let &(_, y_max) = locations.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

    let mut result: u32 = 0;
    for i in 0..=x_max {
        for j in 0..=y_max {
            let mut total_distance = 0;
            for (x, y) in &locations {
                total_distance += (i as i32 - x).abs() + (j as i32 - y).abs();
            }
            if total_distance < TENK {
                result += 1;
            }
        }
    }
    println!("result {}", result);
}
