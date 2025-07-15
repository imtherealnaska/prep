use std::collections::HashSet;

fn contains_key(nums: Vec<i32>) -> bool {
    let uniq_count = nums.iter().collect::<HashSet<_>>().len();
    uniq_count != nums.len()
}
