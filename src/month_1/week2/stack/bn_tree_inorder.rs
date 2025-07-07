use crate::month_1::week2::trees::bst::TreeType;

fn bn_tree_inorder(root: TreeType) -> Vec<i32> {
    let mut result = Vec::new();
    inorder_helper(&root, &mut result);
    result
}

fn inorder_helper(node: &TreeType, result: &mut Vec<i32>) {
    if let Some(n) = node {
        let borrowed = n.borrow();
        inorder_helper(&borrowed.left, result);
        result.push(borrowed.val);
        inorder_helper(&borrowed.right, result);
    }
}
