#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn linked_list_cycle(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return false;
    }

    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;

        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;

        if std::ptr::eq(
            slow.as_ref().unwrap().as_ref(),
            fast.as_ref().unwrap().as_ref(),
        ) {
            return true;
        }
    }
    false
}
