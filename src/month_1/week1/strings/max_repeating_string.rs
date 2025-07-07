fn max_repeating_string(seq: String, w: String) -> i32 {
    let mut k = 0;
    let mut rep_w = String::new();

    loop {
        rep_w.push_str(&w);
        k += 1;

        if seq.contains(&rep_w) {
            //continue
        } else {
            return k - 1;
        }

        if rep_w.len() > seq.len() {
            return k - 1;
        }
    }
}
