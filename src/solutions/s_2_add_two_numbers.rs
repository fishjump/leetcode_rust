/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

pub struct Solution {}
use crate::utils::linked_list::ListNode;

// @lc code=start

use std::borrow::BorrowMut;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = head.borrow_mut();

        let (mut l1, mut l2) = (l1, l2);

        let mut carry = 0;
        while l1.is_some() || l2.is_some() {
            let x = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => 0,
            };

            let y = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => 0,
            };

            let sum = x + y + carry;
            carry = sum / 10;

            *tail = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.as_mut().unwrap().next.borrow_mut();
        }

        if carry > 0 {
            *tail = Some(Box::new(ListNode::new(carry)));
        }

        head
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::add_two_numbers(linked![1, 2, 3], linked![2, 3, 4]),
            linked![3, 5, 7]
        );

        assert_eq!(
            Solution::add_two_numbers(linked![0], linked![0]),
            linked![0]
        );

        assert_eq!(
            Solution::add_two_numbers(linked![9, 9, 9, 9, 9, 9, 9], linked![9, 9, 9, 9]),
            linked![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }
}
