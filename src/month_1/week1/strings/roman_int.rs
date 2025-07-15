fn roman_int(s: String) -> i32 {
    let mut result = 0;
    let mut prev_val = 0;

    // IV => 4 => V - I
    //  VI => 6 => I + V
    for c in s.chars().rev() {
        let current_val = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0, // Handle invalid characters
        };

        if current_val >= prev_val {
            result += current_val;
        } else {
            result -= current_val;
        }
        prev_val = current_val;
    }
    result
}
