use crate::month_1::week2::trees::bst::TreeType;

fn merge_two_bin_trees(mut root1: TreeType, root2: TreeType) -> TreeType {
    match (&root1, &root2) {
        (None, None) => None,
        (None, Some(_)) => root2,
        (Some(_), None) => root1,
        (Some(node1), Some(node2)) => {
            let borrowed2 = node2.borrow();
            // modify in place
            {
                let mut borrowed1 = node1.borrow();
                borrowed1.val += borrowed2.val;

                borrowed1.left = merge_two_bin_trees(borrowed1.left.take(), borrowed2.left.clone());
                borrowed1.right =
                    merge_two_bin_trees(borrowed1.right.take(), borrowed2.right.clone());
            }
            root1
        }
    }
}

fn merge_two_bin_trees_stack(root1: TreeType, root2: TreeType) -> TreeType {
    if root1.is_none() {
        return root2;
    }
    if root1.is_none() {
        return root1;
    }

    let mut stack = vec![(root1.clone(), root2.clone())];

    while let Some((node1, node2)) = stack.pop() {
        if let (Some(n1), Some(n2)) = (node1, node2) {
            let b2 = n2.borrow();
            let mut b1 = n1.borrow_mut();

            b1.val += b2.val;

            if b1.left.is_none() {
                b1.left = b2.left.clone();
            } else if b2.left.is_some() {
                stack.push((b1.left.clone(), b2.left.clone()));
            }
            if b1.right.is_none() {
                b1.right = b2.right.clone();
            } else if b2.left.is_some() {
                stack.push((b1.right.clone(), b2.right.clone()));
            }
        }
    }
    root1
}
