use std::collections::VecDeque;

use super::TreeType;

fn max_depth_dfs(root: TreeType) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut max_depth = 0;
    let mut q = VecDeque::new();

    q.push_back((root.unwrap(), 1));

    while !q.is_empty() {
        let (node, depth) = q.pop_front().unwrap();
        let borrowed = node.borrow();

        max_depth = max_depth.max(depth);

        if let Some(ref left) = borrowed.left {
            q.push_back((left.clone(), depth + 1));
        }
        if let Some(ref right) = borrowed.right {
            q.push_back((right.clone(), depth + 1));
        }
    }
    max_depth
}
