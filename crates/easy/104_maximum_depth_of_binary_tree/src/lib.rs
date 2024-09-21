use std::cell::RefCell;
use std::cmp::max;
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

pub fn depth_first_search(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match node {
        Some(inner) => {
            max(
                depth_first_search(&inner.borrow().left),
                depth_first_search(&inner.borrow().right),
            ) + 1
        }
        None => 0,
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    depth_first_search(&root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let layer_one_l = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let layer_one_r = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        let layer_two_l = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let layer_two_r = Some(Rc::new(RefCell::new(TreeNode::new(7))));

        if let Some(inner) = &layer_one_r {
            inner.borrow_mut().left = layer_two_l;
            inner.borrow_mut().right = layer_two_r;
        }
        if let Some(inner) = &root {
            inner.borrow_mut().left = layer_one_l;
            inner.borrow_mut().right = layer_one_r;
        }
        let result = max_depth(root);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let layer_one_r = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        if let Some(inner) = &root {
            inner.borrow_mut().right = layer_one_r;
        }
        let result = max_depth(root);
        assert_eq!(result, 2);
    }
}
