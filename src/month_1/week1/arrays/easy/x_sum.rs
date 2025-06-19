use std::collections::HashMap;
fn x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let k = k as usize;
    let x = x as usize;
    let n = nums.len();

    if n < k {
        return vec![];
    }

    let mut result = Vec::with_capacity(n - k + 1);

    for i in 0..=n - k {
        let subarray = &nums[i..i + k];
        let x_sum = calculate_x_sum(subarray, x);
        result.push(x_sum);
    }
    result
}

fn calculate_x_sum(subarray: &[i32], x: usize) -> i32 {
    let mut freq: HashMap<i32, usize> = HashMap::new();

    for &num in subarray {
        *freq.entry(num).or_insert(0) += 1;
    }

    if freq.len() < x {
        return subarray.iter().sum();
    }

    let mut freq_vec: Vec<(i32, usize)> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));

    freq_vec
        .into_iter()
        .take(x)
        .map(|(value, count)| value * count as i32)
        .sum()
}
