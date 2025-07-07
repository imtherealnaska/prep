use crate::month_1::week1::arrays::easy::linked_list_cycle::ListNode;

fn sorted_dup_ll(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    let mut head = head;
    let mut current = head.as_mut();

    while let Some(curr_node) = current {
        while curr_node.next.is_some() && curr_node.val == curr_node.next.as_ref().unwrap().val {
            curr_node.next = curr_node.next.as_mut().unwrap().next.take();
        }
        current = curr_node.next.as_mut();
    }
    head
}
