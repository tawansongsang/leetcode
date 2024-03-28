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

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let layer_one_left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = max_depth(root);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = max_depth(root);
        assert_eq!(result, 0);
    }
}
