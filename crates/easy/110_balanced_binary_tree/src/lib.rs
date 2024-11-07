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

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nine = Rc::new(RefCell::new(TreeNode::new(9)));
        let twenty = Rc::new(RefCell::new(TreeNode::new(20)));
        let fifteen = Rc::new(RefCell::new(TreeNode::new(15)));
        let seven = Rc::new(RefCell::new(TreeNode::new(7)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        three.borrow_mut().right = Some(nine.clone());
        three.borrow_mut().left = Some(twenty.clone());
        twenty.borrow_mut().right = Some(seven.clone());
        twenty.borrow_mut().left = Some(fifteen.clone());

        let result = is_balanced(Some(three));
        assert_eq!(result, true);
    }

    // TODO: make test case 2
}
