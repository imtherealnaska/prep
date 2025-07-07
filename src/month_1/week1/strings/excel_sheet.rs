fn excel_sheet(col_title: String) -> i32 {
    let mut result = 0;

    for ch in col_title.chars() {
        let digital_value = (ch as u8 - b'A' + 1) as i32;

        result = result * 26 + digital_value;
    }
    result
}
