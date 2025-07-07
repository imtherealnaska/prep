use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn in_place(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let dummy = Rc::new(RefCell::new(TreeNode::new(0)));
    let mut prev = dummy.clone();

    inorder_inplace(root, &mut prev);
    let result = dummy.borrow().right.clone();
    result
}

fn inorder_inplace(node: Option<Rc<RefCell<TreeNode>>>, prev: &mut Rc<RefCell<TreeNode>>) {
    if let Some(n) = node {
        let left = n.borrow().left.clone();
        let right = n.borrow().right.clone();

        inorder_inplace(left, prev);

        n.borrow_mut().left = None;
        prev.borrow_mut().right = Some(n.clone());
        *prev = n.clone();

        inorder_inplace(right, prev);
    }
}
