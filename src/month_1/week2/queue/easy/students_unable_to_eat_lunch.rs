use std::collections::VecDeque;

fn students_unable_to_eat_lunch(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut q: VecDeque<i32> = students.into_iter().collect();
    let mut stack = sandwiches;

    let mut stack_idx = 0;
    let mut no_match_count = 0;

    while !q.is_empty() && stack_idx < stack.len() {
        let student = q.pop_front().unwrap();

        if student == stack[stack_idx] {
            stack_idx += 1;
            no_match_count = 0;
        } else {
            q.push_back(student);
            no_match_count += 1;

            if no_match_count == q.len() {
                break;
            }
        }
    }
    no_match_count as i32
}
