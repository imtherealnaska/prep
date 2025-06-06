pub fn concat_array(nums: Vec<i32>) -> Vec<i32> {
    std::iter::repeat_n(&nums, 2)
        .flat_map(|v| v.iter())
        .cloned()
        .collect()
}

pub fn concat_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![0; 2 * n];

    // O(n) -> 
    for i in 0..n {
        ans[i] = nums[i];
        ans[i + n] = nums[i];
    }
    ans
}

pub fn concat_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = Vec::with_capacity(2 * n);
    // Still O(n) but O(2n)

    for i in 0..(2 * n) {
        ans.push(nums[i % n]);
    }
    ans
}
