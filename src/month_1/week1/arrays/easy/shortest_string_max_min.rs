fn shortest_string_max_min(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let max_val = *nums.iter().max().unwrap();
    let min_val = *nums.iter().min().unwrap();

    let mut min_length = nums.len();
    let mut last_max = None;
    let mut last_min = None;

    for (i, &num) in nums.iter().enumerate() {
        if num == max_val {
            last_max = Some(i);
            if let Some(min_pos) = last_min {
                min_length = min_length.min(i.abs_diff(min_pos) + 1);
            }
        }

        if num == min_val {
            last_min = Some(i);
            if let Some(max_pos) = last_max {
                min_length = min_length.min(i.abs_diff(max_pos) + 1);
            }
        }
    }
    min_length as i32
}
