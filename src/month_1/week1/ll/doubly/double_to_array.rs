use crate::month_1::week1::ll::singly::LinkedListNode;

fn double_to_array(head: LinkedListNode) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = &head;

    while let Some(node) = current {
        result.push(node.val);
        current = &node.next;
    }
    result
}
