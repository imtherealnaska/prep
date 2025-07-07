use std::collections::HashMap;

fn next_greater(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut next_greater_map = HashMap::new();
    let mut stack = Vec::new();

    for num in nums2 {
        while !stack.is_empty() && num > *stack.last().unwrap() {
            let val = stack.pop().unwrap();
            next_greater_map.insert(val, num);
        }
        stack.push(num);
    }

    nums1
        .iter()
        .map(|&num| *next_greater_map.get(&num).unwrap_or(&-1))
        .collect()
}
