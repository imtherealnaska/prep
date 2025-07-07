use std::collections::HashMap;

fn isomorphic_strings(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_to_t = HashMap::new();
    let mut t_to_s = HashMap::new();

    for (s_char, t_char) in s.chars().zip(t.chars()) {
        match s_to_t.get(&s_char) {
            Some(&mapped_char) => {
                if mapped_char != t_char {
                    return false;
                }
            }

            None => {
                if t_to_s.contains_key(&t_char) {
                    return false;
                }

                s_to_t.insert(s_char, t_char);
                t_to_s.insert(t_char, s_char);
            }
        }
    }
    true
}
