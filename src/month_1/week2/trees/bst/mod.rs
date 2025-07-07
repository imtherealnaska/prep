use std::{cell::RefCell, rc::Rc};

pub mod min_depth;
pub mod max_depth;
pub mod same_tree;
pub mod symmetric_tree;
pub mod sorted_to_tree;

pub type TreeType = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeType,
    pub right: TreeType,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}
