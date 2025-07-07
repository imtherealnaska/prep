use crate::month_1::week2::trees::bst::TreeType;

fn bn_tree_preorder(root: TreeType) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = Vec::new();

    if let Some(node) = root {
        stack.push(node);
    }

    while let Some(node) = stack.pop() {
        let borrowed = node.borrow();

        result.push(borrowed.val);

        if let Some(right) = &borrowed.right {
            stack.push(right.clone());
        }
        if let Some(left) = &borrowed.left {
            stack.push(left.clone());
        }
    }
    result
}
