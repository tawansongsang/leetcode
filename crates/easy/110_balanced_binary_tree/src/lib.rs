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

pub fn check_height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match node {
        Some(n) => {
            let left_height = check_height(n.borrow().left.clone());
            if left_height == -1 {
                return -1;
            }
            let right_height = check_height(n.borrow().right.clone());
            if right_height == -1 {
                return -1;
            }
            if (left_height - right_height).abs() > 1 {
                return -1;
            }
            1 + std::cmp::max(left_height, right_height)
        }
        None => 0,
    }
}

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    if check_height(root) != -1 {
        true
    } else {
        false
    }
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
    #[test]
    fn test_2() {
        let one = Rc::new(RefCell::new(TreeNode::new(1)));
        let two_1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let two_2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let three_1 = Rc::new(RefCell::new(TreeNode::new(3)));
        let three_2 = Rc::new(RefCell::new(TreeNode::new(3)));
        let four_1 = Rc::new(RefCell::new(TreeNode::new(4)));
        let four_2 = Rc::new(RefCell::new(TreeNode::new(4)));
        three_1.borrow_mut().right = Some(four_1.clone());
        three_1.borrow_mut().left = Some(four_2.clone());
        two_1.borrow_mut().right = Some(three_1.clone());
        two_1.borrow_mut().left = Some(three_2.clone());
        one.borrow_mut().right = Some(two_1.clone());
        one.borrow_mut().left = Some(two_2.clone());

        let result = is_balanced(Some(one));
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let result = is_balanced(None);
        assert_eq!(result, true);
    }
}
