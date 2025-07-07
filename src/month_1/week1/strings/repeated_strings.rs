fn repeated_strings(s: String) -> bool {
    let n = s.len();

    if n <= 1 {
        return false;
    }

    let bytes = s.as_bytes();

    for len in 1..=n / 2 {
        if n % 2 != 0 {
            continue;
        }

        let pattern = &bytes[0..len];
        let mut is_valid = true;

        for start in (len..n).step_by(len) {
            if &bytes[start..start + len] != pattern {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            return true;
        }
    }
    false
}
