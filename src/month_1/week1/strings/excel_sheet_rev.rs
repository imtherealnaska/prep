fn excel_sheet_rev(mut col_number: i32) -> String {
    let mut result = String::new();

    while col_number > 0 {
        col_number -= 1;

        let remainder = col_number % 26;

        let ch = (b'A' + remainder as u8) as char;
        result.insert(0, ch);
        col_number /= 26;
    }

    result
}
