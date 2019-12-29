const GRID_DIMENSION: usize = 300;
const SQR: usize = 3;

fn main() {
    println!("{:?}", max_power(grid_init()));
}

fn grid_init() -> [[isize; GRID_DIMENSION]; GRID_DIMENSION] {
    let mut grid = [[0_isize; GRID_DIMENSION]; GRID_DIMENSION];
    for j in 0..GRID_DIMENSION {
        for i in 0..GRID_DIMENSION {
            let rack_id = i + 11;
            let power = (rack_id * (j + 1) + 3999) * rack_id;
            let power = (power as isize / 100) % 10 - 5;
            grid[j][i] = power;
        }
    }
    grid
}

fn max_power(grid: [[isize; GRID_DIMENSION]; GRID_DIMENSION]) -> (isize, isize) {
    let mut max_total = 0;
    let (mut x, mut y) = (0, 0);
    for j in 0..GRID_DIMENSION - SQR {
        for i in 0..GRID_DIMENSION - SQR {
            let mut running_total = 0;
            for k in 0..SQR {
                for l in 0..SQR {
                    running_total += grid[j + l][i + k]
                }
            }
            if max_total < running_total {
                max_total = running_total;
                x = i as isize + 1;
                y = j as isize + 1;
            }
        }
    }
    (x, y)
}
