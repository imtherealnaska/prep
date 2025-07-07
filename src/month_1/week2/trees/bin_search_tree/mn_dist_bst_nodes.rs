use std::i32;

use crate::month_1::week2::trees::bst::TreeType;

fn mn_dist_bst_nodes(root: TreeType) -> i32 {
    let mut min_diff = i32::MAX;
    let mut prev_val: Option<i32> = None;

    inorder_traversal(&root, &mut prev_val, &mut min_diff);
    min_diff
}

fn inorder_traversal(node: &TreeType, prev_val: &mut Option<i32>, min_diff: &mut i32) {
    if let Some(n) = node {
        let borrowed = n.borrow();
        inorder_traversal(&borrowed.left, prev_val, min_diff);

        if let Some(prev) = *prev_val {
            *min_diff = (*min_diff).min(borrowed.val - prev);
        }

        *prev_val = Some(borrowed.val);

        inorder_traversal(&borrowed.right, prev_val, min_diff);
    }
}

