fn invert_miage(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = image;

    for row in &mut result {
        let mut left = 0;
        let mut right = row.len() - 1;

        while left <= right {
            if left == right {
                row[left] = 1 - row[left];
            } else {
                let temp = 1 - row[right];
                row[right] = 1 - row[left];
                row[left] = temp;
            }

            if left >= right {
                break;
            }

            left += 1;
            right -= 1;
        }
    }
    result
}

fn invert_miage_2(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    image
        .into_iter()
        .map(|row| row.into_iter().rev().map(|pixel| 1 - pixel).collect())
        .collect()
}
