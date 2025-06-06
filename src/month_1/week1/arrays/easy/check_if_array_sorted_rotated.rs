pub fn check_if_array_sorted_rotated(nums: Vec<i32>) -> bool {
    let n = nums.len();

    if n <= 1 {
        return true;
    }

    let mut break_count = 0;
    for i in 0..n - 1 {
        if nums[i] > nums[i + 1] {
            break_count += 1;
        }
    }

    match break_count {
        0 => true,
        // If there is rotation then the last element cannot be the largest ,
        1 => nums[n - 1] <= nums[0],
        _ => false,
    }
}

pub fn alt_check_if_array_sorted_rotated(nums: Vec<i32>) -> bool {
    let n = nums.len();

    if n <= 1 {
        return true;
    }

    let break_count = nums
        .windows(2)
        .filter(|window| window[0] > window[1])
        .count();

    match break_count {
        0 => true,
        // If there is rotation then the last element cannot be the largest ,
        1 => nums[n - 1] <= nums[0],
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_array_sorted_rotated() {
        assert_eq!(check_if_array_sorted_rotated(vec![3, 4, 5, 1, 2]), true);
    }
}
