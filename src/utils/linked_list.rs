use std::borrow::BorrowMut;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = head.borrow_mut();

        for x in vec {
            *tail = Some(Box::new(ListNode::new(x)));
            tail = tail.as_mut().unwrap().next.borrow_mut();
        }

        head
    }
}

#[macro_export]
macro_rules! linked {
    [$($e:expr),*] => { crate::utils::linked_list::ListNode::from_vec(vec![$($e.to_owned()), *])};
    [$($e:expr,)*] => { crate::utils::linked_list::ListNode::from_vec(vec![$($e.to_owned()), *])};
}
