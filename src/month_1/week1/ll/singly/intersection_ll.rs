use crate::month_1::week1::arrays::easy::linked_list_cycle::ListNode;

fn intersection_ll(
    head_a: Option<Box<ListNode>>,
    head_b: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut ptr_a = &head_a;
    let mut ptr_b = &head_b;

    while !are_same_node(ptr_a, ptr_b) {
        ptr_a = if ptr_a.is_none() {
            &head_b
        } else {
            &ptr_a.as_ref().unwrap().next
        };

        ptr_b = if ptr_b.is_none() {
            &head_a
        } else {
            &ptr_b.as_ref().unwrap().next
        };
    }
    ptr_a.clone()
}

fn are_same_node(ptr_a: &Option<Box<ListNode>>, ptr_b: &Option<Box<ListNode>>) -> bool {
    match (ptr_a, ptr_b) {
        (None, None) => true,
        (Some(node_a), Some(node_b)) => std::ptr::eq(node_a.as_ref(), node_b.as_ref()),
        _ => false,
    }
}
