rust
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut current = &mut head;
    loop {
        match current {
            None => break,
            Some(node) => {
                if node.val == val {
                    *current = node.next.take();
                } else {
                    current = &mut node.next;
                }
            }
        }
    }
    head
}

