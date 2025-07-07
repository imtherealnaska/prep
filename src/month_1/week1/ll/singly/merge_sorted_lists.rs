use crate::month_1::week1::arrays::easy::linked_list_cycle::ListNode;

use super::LinkedListNode;

fn merge_sorted_lists(list1: LinkedListNode, list2: LinkedListNode) -> LinkedListNode {
    let mut dummy = ListNode::new(0);
    let mut current = &mut dummy;

    let mut l1 = list1;
    let mut l2 = list2;

    while l1.is_some() && l2.is_some() {
        if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
            let next_l1 = l1.as_mut().unwrap().next.take();
            current.next = l1;
            l1 = next_l1;
        } else {
            let next_l2 = l2.as_mut().unwrap().next.take();
            current.next = l2;
            l2 = next_l2;
        }

        current = current.next.as_mut().unwrap();
    }

    current.next = if l1.is_some() { l1 } else { l2 };
    dummy.next
}
