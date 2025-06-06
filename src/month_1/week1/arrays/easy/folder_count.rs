fn folder_count(logs: Vec<String>) -> i32 {
    let mut count = 0;
    for op in logs {
        match op.as_str() {
            "../" => count = std::cmp::max(0, count - 1),
            "./" => {}
            _ => count + 1,
        }
    }
    count
}
