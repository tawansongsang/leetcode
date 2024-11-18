// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut dummy = Box::new(ListNode {
            next: head,
            val: -1,
        });
        let mut curr = &mut dummy;
        while let Some(node) = curr.next.as_mut() {
            if node.val == val {
                curr.next = node.next.take();
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut node_1 = Box::new(ListNode::new(1));
        let mut node_2 = Box::new(ListNode::new(2));
        let mut node_6_1 = Box::new(ListNode::new(6));
        let mut node_3 = Box::new(ListNode::new(3));
        let mut node_4 = Box::new(ListNode::new(4));
        let mut node_5 = Box::new(ListNode::new(5));
        let node_6_2 = Box::new(ListNode::new(6));
        node_1.next = Some(node_2.clone());
        node_2.next = Some(node_6_1.clone());
        node_6_1.next = Some(node_3.clone());
        node_3.next = Some(node_4.clone());
        node_4.next = Some(node_5.clone());
        node_5.next = Some(node_6_2.clone());
        let head = Some(node_1.clone());
        let val = 6;
        let result = Solution::remove_elements(head, val);
        let output = Some(node_1);
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let head = None;
        let val = 1;
        let result = Solution::remove_elements(head, val);
        let output = None;
        assert_eq!(result, output);
    }

    #[test]
    fn test_3() {
        let mut node_7_1 = Box::new(ListNode::new(7));
        let mut node_7_2 = Box::new(ListNode::new(7));
        let mut node_7_3 = Box::new(ListNode::new(7));
        let node_7_4 = Box::new(ListNode::new(7));
        node_7_1.next = Some(node_7_2.clone());
        node_7_2.next = Some(node_7_3.clone());
        node_7_3.next = Some(node_7_4.clone());
        let head = Some(node_7_1);
        let val = 7;
        let result = Solution::remove_elements(head, val);
        let output = None;
        assert_eq!(result, output);
    }
}
