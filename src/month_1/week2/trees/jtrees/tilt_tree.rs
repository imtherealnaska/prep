use crate::month_1::week2::trees::bst::TreeType;

fn tilt_tree(root: TreeType) -> i32 {
    let mut total_tilt = 0;
    calculate_sum_tilt(&root, &mut total_tilt);
    total_tilt
}

fn calculate_sum_tilt(root: &TreeType, total_tilt: &mut i32) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let borrowed = node.borrow();
            let left_sum = calculate_sum_tilt(&borrowed.left, total_tilt);
            let right_sum = calculate_sum_tilt(&borrowed.right, total_tilt);

            let current_tilt = (left_sum - right_sum).abs();

            *total_tilt += current_tilt;

            left_sum + right_sum + borrowed.val
        }
    }
}
