use std::collections::HashMap;

// precond   : nums and target
// post cond : list containing both the indices.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::new();
    // measure of progress : `i` is the measure of progress. If it reaches the end of the vec,
    // The algorithm finishes.
    // LI: for each 'i' we have already looked into all of [..i] elements
    // Loop Invariant (LI): At the start of each iteration, hash_map contains all nums[j] with their indices for all j < i.
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = hash_map.get(&complement) {
            return vec![complement_index as i32, i as i32];
        }
        hash_map.insert(num, i);
    }
    vec![]
}
