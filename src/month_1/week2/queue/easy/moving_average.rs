use std::collections::VecDeque;

#[derive(Debug)]
struct MovingAverage {
    size: usize,
    window: VecDeque<i32>,
    sum: i64,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        MovingAverage {
            size: size as usize,
            window: VecDeque::with_capacity(size as usize),
            sum: 0,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.sum += val as i64;
        self.window.push_back(val);

        if self.window.len() > self.size {
            if let Some(old_val) = self.window.pop_front() {
                self.sum -= old_val as i64;
            }
        }

        self.sum as f64 / self.window.len() as f64
    }
}
