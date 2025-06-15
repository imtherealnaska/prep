use std::cmp::max;

fn width_of_matrix(grid: Vec<Vec<i32>>) -> Vec<i32> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = Vec::new();

    for col in 0..cols {
        let mut max_width = 0;

        for row in 0..rows {
            let num = grid[row][col];
            let width = get_number_width(num);
            max_width = max(max_width, width);
        }
        result.push(max_width);
    }
    result
}

fn get_number_width(num: i32) -> i32 {
    if num == 0 {
        return 1;
    }

    let mut digits = 0;
    let mut n = num.abs();

    while n > 0 {
        digits += 1;
        n /= 10;
    }

    if num < 0 { digits + 1 } else { digits }
}
