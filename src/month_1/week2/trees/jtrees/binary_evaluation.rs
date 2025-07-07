use crate::month_1::week2::trees::bst::TreeType;

fn binary_evaluation(root: TreeType) -> bool {
    match root {
        None => false,
        Some(node) => {
            let borrowed = node.borrow();

            match borrowed.val {
                0 => false,
                1 => true,

                2 => {
                    binary_evaluation(borrowed.left.clone())
                        || binary_evaluation(borrowed.right.clone())
                }
                3 => {
                    binary_evaluation(borrowed.left.clone())
                        && binary_evaluation(borrowed.right.clone())
                }

                _ => panic!("Invalid node {}", borrowed.val),
            }
        }
    }
}
