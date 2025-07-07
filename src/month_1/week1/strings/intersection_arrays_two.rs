use std::collections::HashMap;

fn intersection_arrays_two(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let (count_nums, iter_nums) = if nums1.len() <= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };

    let mut count_map = HashMap::new();
    for num in count_nums {
        *count_map.entry(num).or_insert(0) += 1;
    }

    let mut result = Vec::new();
    for num in iter_nums {
        if let Some(count) = count_map.get_mut(&num) {
            if *count > 0 {
                result.push(num);
                *count -= 1;
            }
        }
    }
    result
}
