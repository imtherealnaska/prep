use std::collections::VecDeque;

use super::TreeType;

fn same_tree(p: TreeType, q: TreeType) -> bool {
    let mut queue = VecDeque::new();

    queue.push_back((p, q));

    while let Some((node_p, node_q)) = queue.pop_front() {
        match (node_p, node_q) {
            (None, None) => continue,
            (None, Some(_)) => return false,
            (Some(_), None) => return false,
            (Some(p_node), Some(q_node)) => {
                let p_b = p_node.borrow();
                let q_b = q_node.borrow();

                if p_b.val != q_b.val {
                    return false;
                }
                queue.push_back((p_b.left.clone(), q_b.left.clone()));
                queue.push_back((p_b.right.clone(), q_b.right.clone()));
            }
        }
    }
    true
}
