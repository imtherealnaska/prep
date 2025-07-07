use super::TreeType;

fn symmetric_tree(root: TreeType) -> bool {
    match root {
        None => true,
        Some(node) => {
            let borrowed = node.borrow();
            helper(&borrowed.left, &borrowed.right)
        }
    }
}

fn helper(left: &TreeType, right: &TreeType) -> bool {
    match (left, right) {
        (None, None) => true,
        (None, Some(_)) | (Some(_), None) => return false,
        (Some(l_n), Some(r_n)) => {
            let l_b = l_n.borrow();
            let r_b = r_n.borrow();

            l_b.val == r_b.val && helper(&l_b.left, &r_b.right) && helper(&l_b.right, &r_b.left)
        }
    }
}
