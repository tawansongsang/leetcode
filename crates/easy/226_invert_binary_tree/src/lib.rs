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
use std::cell::RefCell;
use std::rc::Rc;

trait TreeNodeSwap {
    fn swap(&mut self);
    fn swap_all(&mut self);
}

impl TreeNodeSwap for TreeNode {
    fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }

    fn swap_all(&mut self) {
        self.left.as_mut().map(|node| node.borrow_mut().swap_all());
        self.right.as_mut().map(|node| node.borrow_mut().swap_all());
        self.swap();
    }
}

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.map(|node| {
            node.borrow_mut().swap_all();
            node
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // [4,2,7,1,3,6,9]
        let four = Rc::new(RefCell::new(TreeNode::new(4)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        let seven = Rc::new(RefCell::new(TreeNode::new(7)));
        let one = Rc::new(RefCell::new(TreeNode::new(1)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        let six = Rc::new(RefCell::new(TreeNode::new(6)));
        let nine = Rc::new(RefCell::new(TreeNode::new(9)));
        two.borrow_mut().left = Some(one.clone());
        two.borrow_mut().right = Some(three.clone());
        seven.borrow_mut().left = Some(six.clone());
        seven.borrow_mut().right = Some(nine.clone());
        four.borrow_mut().left = Some(two.clone());
        four.borrow_mut().right = Some(seven.clone());
        let root = Some(four.clone());

        let result = Solution::invert_tree(root);

        // [4,7,2,9,6,3,1]
        two.borrow_mut().right = Some(one.clone());
        two.borrow_mut().left = Some(three.clone());
        seven.borrow_mut().right = Some(six.clone());
        seven.borrow_mut().left = Some(nine.clone());
        four.borrow_mut().right = Some(two.clone());
        four.borrow_mut().left = Some(seven.clone());
        let output = Some(four.clone());

        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        // [2,1,3]
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        let one = Rc::new(RefCell::new(TreeNode::new(1)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        two.borrow_mut().left = Some(one.clone());
        two.borrow_mut().right = Some(three.clone());
        let root = Some(two.clone());

        let result = Solution::invert_tree(root);

        // [2,3,1]
        two.borrow_mut().right = Some(one.clone());
        two.borrow_mut().left = Some(three.clone());
        let output = Some(two.clone());

        assert_eq!(result, output);
    }

    #[test]
    fn test_3() {
        let root = None;

        let result = Solution::invert_tree(root);

        let output = None;

        assert_eq!(result, output);
    }
}
