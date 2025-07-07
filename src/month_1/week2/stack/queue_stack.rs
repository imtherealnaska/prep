use std::{cell::RefCell, collections::VecDeque};

struct MyStack {
    q1: RefCell<VecDeque<i32>>,
    q2: RefCell<VecDeque<i32>>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            q1: RefCell::new(VecDeque::new()),
            q2: RefCell::new(VecDeque::new()),
        }
    }

    fn push(&self, x: i32) {
        let mut q1 = self.q1.borrow_mut();
        let mut q2 = self.q2.borrow_mut();

        q2.push_back(x);

        while let Some(val) = q1.pop_front() {
            q2.push_back(val);
        }

        std::mem::swap(&mut *q1, &mut *q2);
    }

    fn pop(&self) -> i32 {
        self.q1.borrow_mut().pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.q1.borrow().front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q1.borrow().is_empty()
    }
}
