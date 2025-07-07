use super::singly::LinkedListNode;

pub mod double_to_array;

pub struct DLL {
    pub val: i32,
    pub next: LinkedListNode,
    pub prev: LinkedListNode,
}

impl DLL {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }
}
