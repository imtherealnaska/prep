use std::{cell::RefCell, rc::Rc};

use super::{TreeNode, TreeType};

struct Solution;

impl Solution {
    fn sorted_to_tree(nums: Vec<i32>) -> TreeType {
        if nums.is_empty() {
            return None;
        }

        Self::build_bst(&nums, 0, nums.len() - 1)
    }

    fn build_bst(nums: &[i32], left: usize, right: usize) -> TreeType {
        if left < right {
            return None;
        }

        let mid = left + (right - left) / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));

        if mid > 0 {
            root.borrow_mut().left = Self::build_bst(nums, left, mid - 1);
        }
        root.borrow_mut().right = Self::build_bst(nums, mid - 1, right);
        Some(root)
    }
}
