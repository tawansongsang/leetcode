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

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let one = Rc::new(RefCell::new(TreeNode::new(1)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        one.borrow_mut().right = Some(two.clone());
        two.borrow_mut().left = Some(three.clone());
        let result = Solution::preorder_traversal(Some(one));
        let answer = vec![1, 2, 3];
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2() {
        let one = Rc::new(RefCell::new(TreeNode::new(1)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        let four = Rc::new(RefCell::new(TreeNode::new(4)));
        let five = Rc::new(RefCell::new(TreeNode::new(5)));
        let six = Rc::new(RefCell::new(TreeNode::new(6)));
        let seven = Rc::new(RefCell::new(TreeNode::new(7)));
        let eight = Rc::new(RefCell::new(TreeNode::new(8)));
        let nine = Rc::new(RefCell::new(TreeNode::new(9)));
        one.borrow_mut().left = Some(two.clone());
        one.borrow_mut().right = Some(three.clone());
        two.borrow_mut().left = Some(four.clone());
        two.borrow_mut().right = Some(five.clone());
        three.borrow_mut().right = Some(eight.clone());
        five.borrow_mut().left = Some(six.clone());
        five.borrow_mut().right = Some(seven.clone());
        eight.borrow_mut().left = Some(nine.clone());
        let result = Solution::preorder_traversal(Some(one));
        let answer = vec![1, 2, 4, 5, 6, 7, 3, 8, 9];
        assert_eq!(result, answer);
    }

    #[test]
    fn test_3() {
        let result = Solution::preorder_traversal(None);
        let answer = Vec::new();
        assert_eq!(result, answer);
    }

    #[test]
    fn test_4() {
        let one = Rc::new(RefCell::new(TreeNode::new(1)));
        let result = Solution::preorder_traversal(Some(one));
        let answer = vec![1];
        assert_eq!(result, answer);
    }
}
