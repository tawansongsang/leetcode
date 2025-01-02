#![allow(unused)]

// Definition for a binary tree node.
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

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        Solution::traversal_node(root, false)
    }

    fn traversal_node(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if node.is_none() {
            return 0;
        }

        if let Some(n) = node {
            if is_left && n.borrow().left.is_none() && n.borrow().right.is_none() {
                return n.borrow().val;
            }

            return Solution::traversal_node(n.borrow().left.clone(), true)
                + Solution::traversal_node(n.borrow().right.clone(), false);
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Rc::new(RefCell::new(TreeNode::new(3)));
        let nine = Rc::new(RefCell::new(TreeNode::new(9)));
        let twenty = Rc::new(RefCell::new(TreeNode::new(20)));
        let fifteen = Rc::new(RefCell::new(TreeNode::new(15)));
        let seven = Rc::new(RefCell::new(TreeNode::new(7)));
        twenty.borrow_mut().left = Some(fifteen);
        twenty.borrow_mut().right = Some(seven);
        root.borrow_mut().left = Some(nine);
        root.borrow_mut().right = Some(twenty);
        let result = Solution::sum_of_left_leaves(Some(root));
        let expected = 24;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = Solution::sum_of_left_leaves(root);
        let expected = 0;
        assert_eq!(result, expected);
    }
}
