use std::collections::HashSet;

fn intersection_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let (set_nums, iter_nums) = if nums1.len() <= nums2.len() {
        (nums1.into_iter().collect::<HashSet<_>>(), nums2)
    } else {
        (nums2.into_iter().collect::<HashSet<_>>(), nums1)
    };

    let mut result = HashSet::new() ; 
    for num in iter_nums {
        if set_nums.contains(&num) {
            result.insert(num) ; 
        }
    }

    result.into_iter().collect()

}
