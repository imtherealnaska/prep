use std::collections::HashMap;

#[derive(Debug)]
pub struct TwoSum {
    num_count: HashMap<i32, i32>,
}

impl TwoSum {
    pub fn new() -> Self {
        TwoSum {
            num_count: HashMap::new(),
        }
    }

    pub fn add(&mut self, number: i32) {
        *self.num_count.entry(number).or_insert(0) += 1
    }

    pub fn find(&self, value: i32) -> bool {
        for (&num, &count) in &self.num_count {
            let compl = value - num;

            if compl == num {
                if count >= 2 {
                    return true;
                }
            } else {
                if self.num_count.contains_key(&compl) {
                    return true;
                }
            }
        }
        false
    }
}
