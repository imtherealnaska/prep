pub fn diagonal_longest(dimensions: Vec<Vec<i32>>) -> i32 {
    let mut max_diagonal_sq = 0i64;
    let mut max_area = 0;

    for dim in dimensions {
        let l = dim[0] as i64;
        let b = dim[1] as i64;

        let diagonal_sq = l * l + b * b;
        let area = dim[0] * dim[1];

        if diagonal_sq > max_diagonal_sq || (diagonal_sq == max_diagonal_sq && area > max_area) {
            max_diagonal_sq = diagonal_sq;
            max_area = area;
        }
    }
    max_area
}
