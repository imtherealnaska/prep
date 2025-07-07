use std::collections::HashSet;

use crate::month_1::week2::trees::bst::TreeType;

fn two_sum_bst(root: TreeType, k: i32) -> bool {
    let mut seen = HashSet::new();
    dfs_hashset(&root, k, &mut seen)
}

fn dfs_hashset(root: &TreeType, k: i32, seen: &mut HashSet<i32>) -> bool {
    if let Some(node) = root {
        let borrowed = node.borrow();
        let val = borrowed.val;
        let complement = k - val;

        if seen.contains(&complement) {
            return true;
        }

        seen.insert(val);

        dfs_hashset(&borrowed.left, k, seen) || dfs_hashset(&borrowed.right, k, seen)
    } else {
        false
    }
}
