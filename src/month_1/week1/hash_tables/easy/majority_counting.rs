fn majority_counting(nums: Vec<i32>) -> i32 {
    let mut candidate = nums[0];
    let mut count = 1;

    for i in 1..nums.len() {
        if count == 0 {
            candidate = nums[i];
            count = 1;
        } else if nums[i] == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}
