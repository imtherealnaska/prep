use crate::month_1::week2::trees::bst::TreeType;

fn sum_of_left_leaves(root: TreeType) -> i32 {
    dfs_recursive(&root, false)
}

fn dfs_recursive(node: &TreeType, is_left: bool) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let borrowed = n.borrow();

            // is this a leaf
            if is_left && borrowed.right.is_none() && borrowed.left.is_none() {
                return borrowed.val;
            }

            dfs_recursive(&borrowed.left, true) + dfs_recursive(&borrowed.right, false)
        }
    }
}

fn sum_of_left_leaves_iterative(root: TreeType) -> i32 {
    let mut sum = 0;
    let mut stack = Vec::new();

    if let Some(node) = root {
        stack.push((node, false)); // (node , is_left)
    }

    while let Some((node, is_left)) = stack.pop() {
        let borrowed = node.borrow();

        // check if it is a left leaf

        if is_left && borrowed.left.is_none() && borrowed.right.is_none() {
            sum += borrowed.val;
            continue;
        }

        // add children

        if let Some(right) = &borrowed.right {
            stack.push((right.clone(), false));
        }
        if let Some(left) = &borrowed.left {
            stack.push((left.clone(), true));
        }
    }
    sum
}
