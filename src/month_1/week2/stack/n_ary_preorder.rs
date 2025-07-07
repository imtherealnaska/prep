use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            children: Vec::new(),
        }
    }
}

fn preorder_recursive(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
    let mut result = Vec::new();
    preorder_helper(&root, &mut result);
    result
}

fn preorder_helper(root: &Option<Rc<RefCell<Node>>>, result: &mut Vec<i32>) {
    if let Some(node) = root {
        let borrowed = node.borrow();

        result.push(borrowed.val);

        for child in &borrowed.children {
            preorder_helper(&Some(child.clone()), result);
        }
    }
}
