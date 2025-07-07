use crate::month_1::week2::trees::bst::TreeType;

fn range_sum_bst(root: TreeType, low: i32, high: i32) -> i32 {
    range_sum_helper(&root, low, high)
}

fn range_sum_helper(node: &TreeType, low: i32, high: i32) -> i32 {
    if let Some(n) = node {
        let borrowed = n.borrow();
        let val = borrowed.val;

        if val < low {
            return range_sum_helper(&borrowed.right, low, high);
        }

        if val > high {
            return range_sum_helper(&borrowed.left, low, high);
        }

        val + range_sum_helper(&borrowed.left, low, high)
            + range_sum_helper(&borrowed.right, low, high)
    } else {
        0
    }
}
