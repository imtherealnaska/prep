pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows <= 0 {
        return vec![];
    }

    let mut triangle: Vec<Vec<i32>> = Vec::new();

    for row_num in 0..num_rows as usize {
        let mut row = vec![1; row_num + 1];
        (1..row_num).for_each(|col| {
            row[col] = triangle[row_num - 1][col - 1] + triangle[row_num - 1][col];
        });

        triangle.push(row);
    }
    triangle
}

pub fn generate_opt(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows <= 0 {
        return vec![];
    }
    (0..num_rows as usize)
        .scan(Vec::new(), |triangle: &mut Vec<Vec<i32>>, row_num| {
            let row = if row_num == 0 {
                Vec::new()
            } else {
                let prev_row = &triangle[row_num - 1];
                (0..=row_num)
                    .map(|col| {
                        if col == 0 || col == row_num {
                            1
                        } else {
                            prev_row[col - 1] + prev_row[col]
                        }
                    })
                    .collect()
            };

            triangle.push(row.clone());
            Some(row)
        })
        .collect()
}

#[test]
fn test_generate() {
    assert_eq!(
        generate(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ]
    );
}
