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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return Solution::traversal(head, None);
    }

    fn traversal(
        head: Option<Box<ListNode>>,
        mut prev: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut curr) = head {
            let temp = curr.next.take();
            curr.next = prev;
            prev = Some(curr);
            return Solution::traversal(temp, prev);
        }
        return prev;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let five = Some(Box::new(ListNode::new(5)));
        let four = Some(Box::new(ListNode {
            val: 4,
            next: five.clone(),
        }));
        let three = Some(Box::new(ListNode {
            val: 3,
            next: four.clone(),
        }));
        let two = Some(Box::new(ListNode {
            val: 2,
            next: three.clone(),
        }));
        let one = Some(Box::new(ListNode {
            val: 1,
            next: two.clone(),
        }));
        let result = Solution::reverse_list(one);
        let output_1 = Some(Box::new(ListNode::new(1)));
        let output_2 = Some(Box::new(ListNode {
            val: 2,
            next: output_1.clone(),
        }));
        let output_3 = Some(Box::new(ListNode {
            val: 3,
            next: output_2.clone(),
        }));
        let output_4 = Some(Box::new(ListNode {
            val: 4,
            next: output_3.clone(),
        }));
        let output_5 = Some(Box::new(ListNode {
            val: 5,
            next: output_4.clone(),
        }));
        assert_eq!(result, output_5);
    }
    #[test]
    fn test_2() {
        let two = Some(Box::new(ListNode::new(2)));
        let one = Some(Box::new(ListNode {
            val: 1,
            next: two.clone(),
        }));
        let result = Solution::reverse_list(one);
        let output_1 = Some(Box::new(ListNode::new(1)));
        let output_2 = Some(Box::new(ListNode {
            val: 2,
            next: output_1.clone(),
        }));
        assert_eq!(result, output_2);
    }
    #[test]
    fn test_3() {
        let result = Solution::reverse_list(None);
        assert_eq!(result, None);
    }
}
