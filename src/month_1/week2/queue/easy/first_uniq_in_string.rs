use std::collections::HashMap;

fn first_uniq_in_string(s: String) -> i32 {
    let mut char_count = HashMap::new();

    for ch in s.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    for (i, ch) in s.chars().enumerate() {
        if char_count[&ch] == 1 {
            return i as i32;
        }
    }
    -1
}
