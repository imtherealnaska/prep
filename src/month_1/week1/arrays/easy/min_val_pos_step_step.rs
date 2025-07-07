fn min_val_pos_step_step(nums: Vec<i32>) -> i32 {
    let mut min_sum = 0;
    let mut current_sum = 0;

    for num in nums {
        current_sum += num;
        min_sum = min_sum.min(current_sum);
    }

    if min_sum >= 0 { 1 } else { 1 - min_sum }
}
