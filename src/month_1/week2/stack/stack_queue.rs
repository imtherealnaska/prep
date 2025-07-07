use std::cell::RefCell;

struct MyQueue {
    in_stack: RefCell<Vec<i32>>,
    out_stack: RefCell<Vec<i32>>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            in_stack: RefCell::new(Vec::new()),
            out_stack: RefCell::new(Vec::new()),
        }
    }

    fn push(&self, x: i32) {
        self.in_stack.borrow_mut().push(x);
    }

    fn pop(&self) -> i32 {
        self.transfer_if_needed();
        self.out_stack.borrow_mut().pop().unwrap()
    }

    fn peek(&self) -> i32 {
        self.transfer_if_needed();
        *self.out_stack.borrow().last().unwrap()
    }
    fn empty(&self) -> bool {
        self.in_stack.borrow().is_empty() && self.out_stack.borrow().is_empty()
    }

    fn transfer_if_needed(&self) {
        if self.out_stack.borrow().is_empty() {
            let mut in_stack = self.in_stack.borrow_mut();
            let mut out_stack = self.out_stack.borrow_mut();

            while let Some(val) = in_stack.pop() {
                out_stack.push(val);
            }
        }
    }
}
