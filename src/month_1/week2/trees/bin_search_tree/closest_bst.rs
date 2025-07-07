use crate::month_1::week2::trees::bst::TreeType;

fn closest_bst_recursive(root: TreeType, target: f64) -> i32 {
    if let Some(node) = root {
        // determince which subtree to explore
        let borrowed = node.borrow();
        let val = borrowed.val as f64;

        let next_subtree = if target < val {
            &borrowed.left
        } else {
            &borrowed.right
        };

        if next_subtree.is_none() {
            return borrowed.val;
        }

        let closest_n_subtree = closest_bst_recursive(next_subtree.clone(), target);

        let current_diff = (val - target).abs();
        let subtree_diff = (closest_n_subtree as f64 - target).abs();

        if current_diff < subtree_diff {
            borrowed.val
        } else if current_diff > subtree_diff {
            closest_n_subtree
        } else {
            borrowed.val.min(closest_n_subtree)
        }
        // if  reacghes the leaf , current node is the answer
        // recursion
    } else {
        0
    }
}
