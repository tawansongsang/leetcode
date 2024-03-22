// Definition for singly-linked list.
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

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head == None {
        return None;
    }
    let node_head = head.as_ref().unwrap();
    let mut node_current = node_head.clone();
    let mut node_next = node_head.next.as_ref();
    while !node_next.is_none() {
        if node_current.val == node_next.unwrap().val {
            node_next = node_next.unwrap().next.as_ref();
            continue;
        }

        node_current.next = node_next.cloned();
        node_current = node_next.unwrap().clone();
        node_next = node_next.unwrap().next.as_ref();
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut in_node_1 = Box::new(ListNode::new(1));
        let mut in_node_2 = Box::new(ListNode::new(1));
        in_node_1.next = Some(in_node_2.clone());
        let in_node_3 = Box::new(ListNode::new(2));
        in_node_2.next = Some(in_node_3.clone());
        let input = in_node_1.clone();
        let result = delete_duplicates(Some(input));
        let mut o_node_1 = Box::new(ListNode::new(1));
        let o_node_2 = Box::new(ListNode::new(2));
        o_node_1.next = Some(o_node_2.clone());
        let output = Some(o_node_1.clone());
        assert_eq!(result, output);
    }
}
