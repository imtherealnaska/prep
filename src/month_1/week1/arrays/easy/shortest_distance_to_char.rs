pub fn shortest_distance_to_char(s: String, c: char) -> Vec<i32> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    let c_pos: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter_map(|(i, &ch)| if ch == c { Some(i) } else { None })
        .collect();

    let mut answer = Vec::with_capacity(n);

    for i in 0..n {
        let min_distance = c_pos
            .iter()
            .map(|&pos| i.abs_diff(pos) as i32)
            .min()
            .unwrap();

        answer.push(min_distance);
    }
    answer
}
