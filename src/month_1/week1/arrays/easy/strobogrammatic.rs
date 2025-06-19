fn strobogrammatic(num: String) -> bool {
    let chars: Vec<char> = num.chars().collect();
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        let left_char = chars[left];
        let right_char = chars[right];

        if !is_valid_pair(left_char, right_char) {
            return false;
        }

        if left >= right {
            break;
        }

        left += 1;
        right -= 1;
    }
    true
}

fn is_valid_pair(left: char, right: char) -> bool {
    match (left, right) {
        ('0', '0') => true,
        ('1', '1') => true,
        ('6', '9') => true,
        ('8', '8') => true,
        ('9', '6') => true,
        _ => false,
    }
}
