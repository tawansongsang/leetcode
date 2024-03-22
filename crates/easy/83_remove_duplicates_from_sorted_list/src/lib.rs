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

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head == None {
        return None;
    }
    let mut current_opt = head.as_mut();
    while let Some(current) = current_opt {
        let mut next_opt = current.next.take();

        while let Some(next) = next_opt.as_mut() {
            if current.val == next.val {
                next_opt = next.next.take();
            } else {
                current.next = next_opt;
                break;
            }
        }

        current_opt = current.next.as_mut();
    }

    head
}

pub fn delete_duplicates_best_time(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = &mut head;
    while let Some(node) = cur {
        while let Some(next) = &mut node.next {
            if node.val == next.val {
                node.next = next.next.take();
            } else {
                break;
            }
        }
        cur = &mut node.next;
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
        let in_node_3 = Box::new(ListNode::new(2));
        in_node_2.next = Some(in_node_3);
        in_node_1.next = Some(in_node_2);
        println!("{:?}", in_node_1);
        let result = delete_duplicates(Some(in_node_1));
        let mut o_node_1 = Box::new(ListNode::new(1));
        let o_node_2 = Box::new(ListNode::new(2));
        o_node_1.next = Some(o_node_2);
        assert_eq!(result, Some(o_node_1));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_2() {
        let mut in_node_1 = Box::new(ListNode::new(1));
        let mut in_node_2 = Box::new(ListNode::new(1));
        let mut in_node_3 = Box::new(ListNode::new(2));
        let mut in_node_4 = Box::new(ListNode::new(3));
        let in_node_5 = Box::new(ListNode::new(3));
        in_node_4.next = Some(in_node_5);
        in_node_3.next = Some(in_node_4);
        in_node_2.next = Some(in_node_3);
        in_node_1.next = Some(in_node_2);
        println!("{:?}", in_node_1);
        let result = delete_duplicates(Some(in_node_1));
        let mut o_node_1 = Box::new(ListNode::new(1));
        let mut o_node_2 = Box::new(ListNode::new(2));
        let o_node_3 = Box::new(ListNode::new(3));
        o_node_2.next = Some(o_node_3);
        o_node_1.next = Some(o_node_2);
        assert_eq!(result, Some(o_node_1));
        // assert_eq!(result, None);
    }
}
