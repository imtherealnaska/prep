use crate::month_1::week2::trees::bst::TreeType;

fn average_level(root: TreeType) -> Vec<f64> {
    use std::collections::VecDeque;
    if root.is_none() {
        return vec![];
    }

    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let level_sze = queue.len();
        let mut level_sum: i64 = 0;

        for _ in 0..level_sze {
            let node = queue.pop_front().unwrap();
            let borrowed = node.borrow();

            level_sum += borrowed.val as i64;

            if let Some(left) = &borrowed.left {
                queue.push_back(left.clone());
            }

            if let Some(right) = &borrowed.right {
                queue.push_back(right.clone());
            }
        }

        result.push(level_sum as f64 / level_sze as f64);
    }
    result
}
