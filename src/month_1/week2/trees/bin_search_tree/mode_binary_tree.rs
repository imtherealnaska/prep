use std::collections::HashMap;

use crate::month_1::week2::trees::bst::TreeType;

fn mode_binary_tree(root: TreeType) -> Vec<i32> {
    let mut count_map = HashMap::new();
    count_freq(&root, &mut count_map);

    let max_freq = count_map.values().max().copied().unwrap_or(0);

    count_map
        .into_iter()
        .filter(|(_, freq)| *freq == max_freq)
        .map(|(val, _)| val)
        .collect()
}

fn count_freq(root: &TreeType, count_map: &mut HashMap<i32, i32>) {
    if let Some(n) = root {
        let borrowed = n.borrow();
        *count_map.entry(borrowed.val).or_insert(0) += 1;
        count_freq(&borrowed.left, count_map);
        count_freq(&borrowed.right, count_map);
    }
}
