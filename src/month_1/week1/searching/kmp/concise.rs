fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let n = text.len();
    let m = pattern.len();

    if m == 0 {
        return (0..=n).collect();
    }

    if n == 0 || m > n {
        return vec![];
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();

    let lps = lps(pattern_bytes);

    let mut i = 0; // text 
    let mut j = 0; // pattern 
    let mut matches = Vec::new();

    while i < n {
        if pattern_bytes[j] == text_bytes[i] {
            i += 1;
            j += 1;
        }
        if j == m {
            matches.push(i - j);
            j = lps[j - 1];
        } else if i < n && pattern_bytes[j] != text_bytes[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    matches
}

pub fn lps(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let mut lps = vec![0; m];

    let mut length = 0;
    let mut i = 1;

    while i < m {
        if pattern[i] == pattern[length] {
            length += 1;
            lps[i] = length;
            length += 1;
        } else if length != 0 {
            length = lps[length - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    lps
}
