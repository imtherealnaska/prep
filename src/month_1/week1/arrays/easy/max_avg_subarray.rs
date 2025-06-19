fn max_avg_subarray(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let n = nums.len();

    if n < k {
        return 0.0;
    }

    let mut window_sum: i64 = nums[0..k].iter().map(|&x| x as i64).sum();
    let mut max_sum = window_sum;

    for i in k..n {
        window_sum = window_sum - nums[i - k] as i64 + nums[i] as i64;
        max_sum = max_sum.max(window_sum);
    }
    max_sum as f64 / k as f64
}
