use std::{collections::VecDeque, i32};

use super::TreeType;

fn min_depth_rec(root: TreeType) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let borrowed = node.borrow();
            if borrowed.left.is_none() && borrowed.right.is_none() {
                return 1;
            }

            let mut min_depth = i32::MAX;
            if let Some(ref node) = borrowed.left {
                min_depth = min_depth.min(min_depth_rec(Some(node.clone())));
            }
            if let Some(ref node) = borrowed.right {
                min_depth = min_depth.min(min_depth_rec(Some(node.clone())));
            }
            min_depth + 1
        }
    }
}

fn min_depth_bfs(root: TreeType) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut queue = VecDeque::new();
    queue.push_back((root.unwrap(), 1));

    while let Some((node, depth)) = queue.pop_front() {
        let borrowed = node.borrow();

        if borrowed.left.is_none() && borrowed.right.is_none() {
            return depth;
        }

        if let Some(ref node) = borrowed.left {
            queue.push_back((node.clone(), depth + 1));
        }
        if let Some(ref node) = borrowed.right {
            queue.push_back((node.clone(), depth + 1));
        }
    }
    0
}
