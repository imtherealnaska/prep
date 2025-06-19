use std::collections::HashSet;

fn contains_dup_2(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut window: HashSet<i32> = HashSet::new();

    for (i, &num) in nums.iter().enumerate() {
        if nums.contains(&num) {
            return true;
        }

        window.insert(num);

        while window.len() > k {
            window.remove(&nums[i - k]);
        }
    }
    false
}
