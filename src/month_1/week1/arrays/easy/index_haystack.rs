fn index_haystack(haystack: String, needle: String) -> i32 {
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();
    let needle_len = needle.len();

    if needle_len == 0 {
        return 0;
    }

    if needle_len > haystack.len() {
        return -1;
    }

    for i in 0..=haystack.len() - needle_len {
        if haystack_bytes[i..i + needle_len] == needle_bytes[..] {
            return i as i32;
        }
    }
    -1
}
