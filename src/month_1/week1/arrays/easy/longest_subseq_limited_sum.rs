fn longest_subseq_limited_sum(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut sorted_nums = nums;
    sorted_nums.sort_unstable();

    let mut prefx_sums = vec![0; sorted_nums.len() + 1];

    for i in 0..sorted_nums.len() {
        prefx_sums[i + 1] = prefx_sums[i] + sorted_nums[i];
    }

    queries
        .iter()
        .map(|&query| match prefx_sums.binary_search(&(&query + 1)) {
            Ok(pos) => (pos - 1) as i32,
            Err(pos) => (pos - 1) as i32,
        })
        .collect()
}
