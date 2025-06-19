fn min_pos_sum_array(nums: Vec<i32>, l: i32, r: i32) -> i32 {
    let l = l as usize;
    let r = r as usize;
    let n = nums.len();

    if n < l {
        return -1;
    }

    let mut min_pos_sum = i32::MAX;
    let mut found = false;

    for length in l..=r.min(n) {
        let mut current_sum: i32 = nums[0..length].iter().sum();

        if current_sum > 0 {
            min_pos_sum = min_pos_sum.min(current_sum);
            found = true;
        }

        for i in length..n {
            current_sum = current_sum - nums[i - length] + nums[i];

            if current_sum > 0 {
                min_pos_sum = min_pos_sum.min(current_sum);
                found = true;
            }
        }
    }
    if found { min_pos_sum } else { -1 }
}
