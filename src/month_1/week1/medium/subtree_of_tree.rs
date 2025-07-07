use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn subtree_of_tree(root: TreeLink, sub_root: TreeLink) -> bool {
    if root.is_none() {
        return false;
    }

    if is_same_tree(root.clone(), sub_root.clone()) {
        return true;
    }

    let root_node = root.as_ref().unwrap().borrow();

    is_same_tree(root_node.left.clone(), sub_root.clone())
        || is_same_tree(root_node.right.clone(), sub_root.clone())
}

fn is_same_tree(
    clone_1: Option<Rc<RefCell<TreeNode>>>,
    clone_2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (clone_1, clone_2) {
        (None, None) => true,
        (None, Some(_)) | (Some(_), None) => false,
        (Some(root), Some(sub_root)) => {
            let n1 = root.borrow();
            let n2 = sub_root.borrow();

            n1.val == n2.val
                && is_same_tree(n1.left.clone(), n2.left.clone())
                && is_same_tree(n1.right.clone(), n2.right.clone())
        }
    }
}
