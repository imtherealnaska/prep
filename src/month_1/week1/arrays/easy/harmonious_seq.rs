use std::collections::HashMap;

fn harmonious_seq(nums: Vec<i32>) -> i32 {
    let mut freq: HashMap<i32, i32> = HashMap::new();

    for &num in &nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut maxc_length = 0;

    for (&num, &count) in &freq {
        if let Some(next_count) = freq.get(&(num + 1)) {
            maxc_length = maxc_length.max(count + next_count);
        }
    }
    maxc_length
}
