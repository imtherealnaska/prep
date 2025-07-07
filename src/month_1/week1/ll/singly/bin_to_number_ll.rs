use crate::month_1::week1::arrays::easy::linked_list_cycle::ListNode;

fn bin_to_number_ll(head: Option<Box<ListNode>>) -> i32 {
    let mut result = 0;
    let mut current = &head;

    while let Some(node) = current {
        result = result * 2 + node.val;
        current = &node.next;
    }
    result
}
