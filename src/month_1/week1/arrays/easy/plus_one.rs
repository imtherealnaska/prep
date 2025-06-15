pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in (0..digits.len()).rev() {
        if digits[i] < 9 {
            digits[i] += 1;
            return digits;
        } else {
            digits[i] = 0;
        }
    }

    let mut result = Vec::with_capacity(digits.len() + 1);
    result.push(1);
    result.extend(digits);
    result
}

fn plus_one_inplace(mut digits: Vec<i32>) -> Vec<i32> {
    let n = digits.len();

    digits[n - 1] += 1;

    for i in (0..n).rev() {
        if digits[i] >= 10 {
            digits[i] = 0;

            if i == 0 {
                digits.insert(0, 1);
                break;
            } else {
                digits[i - 1] += 1;
            }
        } else {
            break;
        }
    }
    digits
}
