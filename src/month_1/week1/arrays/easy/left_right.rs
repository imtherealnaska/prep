// O(n) + O(n) + O(n)
pub fn left_right(nums: Vec<i32>) -> Vec<i32> {
    // left sum array
    let n = nums.len();
    let mut left_sum = vec![0; n];
    let mut right_sum = vec![0; n];

    for i in 1..n {
        left_sum[i] = left_sum[i - 1] + nums[i - 1];
    }
    for i in (0..n - 1).rev() {
        right_sum[i] = right_sum[i + 1] + nums[i + 1];
    }

    let mut answer = vec![0; n];

    for i in 0..n {
        answer[i] = (left_sum[i] - right_sum[i]).abs();
    }

    answer
}

pub fn left_right_two_pass(nums: Vec<i32>) -> Vec<i32> {
    // left sum array
    let n = nums.len();
    let mut answer = vec![0; n];

    for i in 1..n {
        answer[i] = answer[i - 1] + nums[i - 1];
    }

    let mut right_sum = 0;

    for i in (0..n).rev() {
        let left_sum = answer[i];
        answer[i] = (left_sum - right_sum).abs();
        right_sum += nums[i];
    }

    answer
}

pub fn one_pass_soln(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let t_sum: i32 = nums.iter().sum();
    let mut answer = vec![0; n];

    let mut left_sum = 0;

    for i in 0..n {
        let right_sum = t_sum - left_sum - nums[i];
        answer[i] = (left_sum - right_sum).abs();
        left_sum += nums[i];
    }

    answer
}
