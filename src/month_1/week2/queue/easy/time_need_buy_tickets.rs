use std::collections::VecDeque;

fn time_need_buy_tickets_deque(tickets: Vec<i32>, k: i32) -> i32 {
    let mut q: VecDeque<(usize, i32)> = tickets.into_iter().enumerate().collect();

    let mut time = 0;

    let target = k as usize;

    while let Some((indx, rem)) = q.pop_front() {
        time += 1;

        if indx == target && rem == 0 {
            return time;
        }

        if rem > 1 {
            q.push_back((indx, rem - 1));
        }
    }

    time
}
