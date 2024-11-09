use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left_node = node.borrow().left.clone();
                let right_node = node.borrow().right.clone();
                if left_node.is_none() && right_node.is_none() {
                    0 + 1
                } else if left_node.is_none() {
                    Solution::min_depth(right_node) + 1
                } else if right_node.is_none() {
                    Solution::min_depth(left_node) + 1
                } else {
                    Solution::min_depth(left_node).min(Solution::min_depth(right_node)) + 1
                }
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        let nine = Rc::new(RefCell::new(TreeNode::new(9)));
        let twenty = Rc::new(RefCell::new(TreeNode::new(20)));
        let fifteen = Rc::new(RefCell::new(TreeNode::new(15)));
        let seven = Rc::new(RefCell::new(TreeNode::new(7)));
        three.borrow_mut().left = Some(nine.clone());
        three.borrow_mut().right = Some(twenty.clone());
        twenty.borrow_mut().left = Some(fifteen.clone());
        twenty.borrow_mut().right = Some(seven.clone());

        let result = Solution::min_depth(Some(three));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        let four = Rc::new(RefCell::new(TreeNode::new(4)));
        let five = Rc::new(RefCell::new(TreeNode::new(5)));
        let six = Rc::new(RefCell::new(TreeNode::new(6)));
        two.borrow_mut().right = Some(three.clone());
        three.borrow_mut().right = Some(four.clone());
        four.borrow_mut().right = Some(five.clone());
        five.borrow_mut().right = Some(six.clone());

        let result = Solution::min_depth(Some(two));
        assert_eq!(result, 5);
    }

    #[test]
    fn test_32() {
        let one = Rc::new(RefCell::new(TreeNode::new(1)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        let four = Rc::new(RefCell::new(TreeNode::new(4)));
        let five = Rc::new(RefCell::new(TreeNode::new(5)));
        one.borrow_mut().left = Some(two.clone());
        one.borrow_mut().right = Some(three.clone());
        two.borrow_mut().left = Some(four.clone());
        two.borrow_mut().right = Some(five.clone());

        let result = Solution::min_depth(Some(one));
        assert_eq!(result, 2);
    }
}
