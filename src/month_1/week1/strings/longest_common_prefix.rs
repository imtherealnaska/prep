// https://leetcode.com/problems/longest-common-prefix/?envType=problem-list-v2&envId=string
fn longest_common_prefix(strs: Vec<String>) -> String {
    let str_chars: Vec<Vec<char>> = strs.iter().map(|c| c.chars().collect()).collect();

    let min_len = str_chars.iter().map(|c| c.len()).min().unwrap_or(0);

    let mut result = String::new();

    for i in 0..min_len {
        let current_char = str_chars[0][i];

        if str_chars.iter().all(|chars| chars[i] == current_char) {
            result.push(current_char);
        } else {
            break;
        }
    }

    result
}
