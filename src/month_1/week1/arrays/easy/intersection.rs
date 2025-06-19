use super::linked_list_cycle::ListNode;

fn intersection_of_two_arrays(
    head_a: Option<Box<ListNode>>,
    head_b: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if head_a.is_none() || head_b.is_none() {
        return None;
    }

    let mut pointer_a = &head_a;
    let mut pointer_b = &head_b;

    loop {
        match (pointer_a, pointer_b) {
            (Some(node_a), Some(node_b)) => {
                if std::ptr::eq(node_a.as_ref(), node_b.as_ref()) {
                    return pointer_a.clone();
                }
            }
            (None, None) => {
                return None;
            }
            _ => {}
        }

        pointer_a = match pointer_a {
            Some(node) => &node.next,
            None => &head_b,
        };

        pointer_b = match pointer_b {
            Some(node) => &node.next,
            None => &head_a,
        };
    }
}
