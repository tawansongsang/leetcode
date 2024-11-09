use std::cell::RefCell;
use std::collections::VecDeque;
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let result = target_sum - node.borrow().val;
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();

                if left.is_none() && right.is_none() && result == 0 {
                    return true;
                }

                Solution::has_path_sum(left, result) || Solution::has_path_sum(right, result)
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let five = Rc::new(RefCell::new(TreeNode::new(5)));
        let four_1 = Rc::new(RefCell::new(TreeNode::new(4)));
        let eight = Rc::new(RefCell::new(TreeNode::new(8)));
        let eleven = Rc::new(RefCell::new(TreeNode::new(11)));
        let thirteen = Rc::new(RefCell::new(TreeNode::new(13)));
        let four_2 = Rc::new(RefCell::new(TreeNode::new(4)));
        let seven = Rc::new(RefCell::new(TreeNode::new(7)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        let one = Rc::new(RefCell::new(TreeNode::new(1)));

        five.borrow_mut().left = Some(four_1.clone());
        five.borrow_mut().right = Some(eight.clone());
        four_1.borrow_mut().left = Some(eleven.clone());
        eight.borrow_mut().left = Some(thirteen.clone());
        eight.borrow_mut().right = Some(four_2.clone());
        eleven.borrow_mut().left = Some(seven.clone());
        eleven.borrow_mut().right = Some(two.clone());
        four_2.borrow_mut().right = Some(one.clone());
        let result = Solution::has_path_sum(Some(five), 22);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let one = Rc::new(RefCell::new(TreeNode::new(1)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        one.borrow_mut().left = Some(two.clone());
        one.borrow_mut().right = Some(three.clone());

        let result = Solution::has_path_sum(Some(one), 5);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let result = Solution::has_path_sum(None, 0);
        assert_eq!(result, false);
    }
}
