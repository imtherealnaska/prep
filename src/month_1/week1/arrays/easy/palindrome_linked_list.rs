use super::linked_list_cycle::ListNode;

fn palindrome_linked_list_best(head: Option<Box<ListNode>>) -> bool {
    let mut len = 0;
    let mut current = &head;

    while let Some(node) = current {
        len += 1;
        current = &node.next;
    }

    if len <= 1 {
        return true;
    }

    let mid = len / 2;
    let mut current = &head;

    for _ in 0..mid {
        current = &current.as_ref().unwrap().next;
    }

    let mut second_half = reverse_list(current.clone());
    let mut first_half = &head;

    for _ in 0..mid {
        if first_half.as_ref().unwrap().val != second_half.as_ref().unwrap().val {
            return false;
        }

        first_half = &first_half.as_ref().unwrap().next;
        second_half = second_half.unwrap().next;
    }

    true
}

fn palindrome_linked_list(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return true;
    }

    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    let mut second_half = reverse_list(slow.clone());
    let mut first_half = &head;

    while second_half.is_some() {
        if first_half.as_ref().unwrap().val != second_half.as_ref().unwrap().val {
            return false;
        }

        first_half = &first_half.as_ref().unwrap().next;
        second_half = second_half.unwrap().next;
    }
    true
}

fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut current) = head {
        // split and operate
        let next = current.next.take();
        current.next = prev;

        //update the prev and head
        prev = Some(current);
        head = next;
    }
    prev
}
