fn sort_parity(mut a: Vec<i32>) -> Vec<i32> {
    a.sort_by(|x, y| (x % 2).cmp(&(y % 2)));
    a
}

fn sort_parity_two(mut a: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = a.len() - 1;

    while i < j {
        match (a[i] % 2, a[j] % 2) {
            (1, 0) => {
                a.swap(i, j);
                i += 1;
                j -= 1;
            }
            (0, _) => {
                i += 1;
            }
            (1, 1) => {
                j -= 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    a
}
