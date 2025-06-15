pub fn interval_problem_meeting(mut intervals: Vec<Vec<i32>>) -> bool {
    if intervals.len() <= 1 {
        return true;
    }
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    for i in 1..intervals.len() {
        let prev = intervals[i - 1][1];
        let curr_start = intervals[i][0];

        if prev > curr_start {
            return false;
        }
    }
    false
}
