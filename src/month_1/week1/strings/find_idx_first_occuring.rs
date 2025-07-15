fn find_idx_first_occuring(haystack: String, needle: String) -> i32 {
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    let needle_len = needle_bytes.len();

    for i in 0..=haystack.len() - needle_len {
        if haystack_bytes[i..i + needle_len] == needle_bytes[..] {
            return i as i32;
        }
    }
    -1
}

fn find_idx_first_occuring_kmp(haystack: String, needle: String) -> i32 {
    let haystach_chars: Vec<char> = haystack.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();

    let lps = build_lps(&needle_chars);

    let mut i = 0;
    let mut j = 0;

    while i < haystach_chars.len() {
        if haystach_chars[i] == needle_chars[i] {
            i += 1;
            j += 1;
        }

        if j == needle_chars.len() {
            return (i - j) as i32;
        } else if i < haystach_chars.len() && haystach_chars[i] != needle_chars[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    -1
}

fn build_lps(pattern: &[char]) -> Vec<usize> {
    let m = pattern.len();
    let mut lps = vec![0; m];
    let mut len = 0;
    let mut i = 1;

    while i < m {
        // waits for the first one to match.
        if pattern[i] == pattern[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    lps
}
