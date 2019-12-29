const GRID_DIMENSION: usize = 301;
const SQR: usize = 3;

fn main() {
    println!("{:?}", max_power(grid_init()));
}

fn grid_init() -> [[isize; GRID_DIMENSION]; GRID_DIMENSION] {
    let mut grid = [[0_isize; GRID_DIMENSION]; GRID_DIMENSION];
    for j in 1..GRID_DIMENSION {
        for i in 1..GRID_DIMENSION {
            let rack_id = i + 10;
            let power = (rack_id * j + 3999) * rack_id;
            let power = (power as isize / 100) % 10 - 5;
            grid[j][i] = power + grid[j - 1][i] + grid[j][i - 1] - grid[j - 1][i - 1];
        }
    }
    grid
}

fn max_power(grid: [[isize; GRID_DIMENSION]; GRID_DIMENSION]) -> (isize, isize) {
    let mut max_total = 0;
    let (mut x, mut y) = (0, 0);
    for j in SQR..GRID_DIMENSION {
        for i in SQR..GRID_DIMENSION {
            let running_total =
                grid[j][i] - grid[j - SQR][i] - grid[j][i - SQR] + grid[j - SQR][i - SQR];

            if max_total < running_total {
                max_total = running_total;
                x = (i - SQR + 1) as isize;
                y = (j - SQR + 1) as isize;
            }
        }
    }
    (x, y)
}
