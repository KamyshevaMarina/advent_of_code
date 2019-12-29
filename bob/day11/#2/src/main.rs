const GRID_DIMENSION: usize = 301;

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

fn max_power(grid: [[isize; GRID_DIMENSION]; GRID_DIMENSION]) -> (isize, isize, isize) {
    let mut max_total = 0;
    let (mut x, mut y, mut s) = (0, 0, 0);
    for z in 1..GRID_DIMENSION {
        for j in z..GRID_DIMENSION {
            for i in z..GRID_DIMENSION {
                let running_total =
                    grid[j][i] - grid[j - z][i] - grid[j][i - z] + grid[j - z][i - z];
                if max_total < running_total {
                    max_total = running_total;
                    x = (i - z + 1) as isize;
                    y = (j - z + 1) as isize;
                    s = z as isize;
                }
            }
        }
    }
    (x, y, s)
}
