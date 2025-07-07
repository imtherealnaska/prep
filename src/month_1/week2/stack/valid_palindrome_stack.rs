use crate::month_1::week1::ll::singly::LinkedListNode;

fn valid_palindrome_stack(head: LinkedListNode) -> bool {
    let mut values = Vec::new();
    let mut current = head.as_ref();

    while let Some(node) = current {
        values.push(node.val);
        current = node.next.as_ref();
    }

    let len = values.len();
    for i in 0..len / 2 {
        if values[i] != values[len - i - 1] {
            return false;
        }
    }
    true
}
