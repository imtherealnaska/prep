fn abbreviation(word: String, abbr: String) -> bool {
    let w_bytes = word.as_bytes();
    let a_bytes = abbr.as_bytes();

    let mut i = 0; // word pointer 
    let mut j = 0; // abbr pointer 

    while j < a_bytes.len() {
        if a_bytes[j].is_ascii_digit() {
            if a_bytes[j] == b'0' {
                return false;
            }
            let mut num = 0;
            while j < a_bytes.len() && a_bytes[j].is_ascii_digit() {
                num = num * 10 + (a_bytes[j] - b'0') as usize;
                j += 1;
            }
            i += num;
        } else {
            if i >= word.len() || a_bytes[j] != w_bytes[i] {
                return false;
            }

            i += 1;
            j += 1;
        }
    }
    i == word.len()
}
