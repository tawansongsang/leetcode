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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(r) => is_same_node(&r.borrow().left, &r.borrow().right),
        None => true,
    }
}

pub fn is_same_node(l: &Option<Rc<RefCell<TreeNode>>>, r: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (l, r) {
        (None, None) => true,
        (None, _) | (_, None) => false,
        (Some(n1), Some(n2)) if n1.borrow().val != n2.borrow().val => false,
        (Some(n1), Some(n2)) => {
            is_same_node(&n1.borrow().left, &n2.borrow().right)
                && is_same_node(&n1.borrow().right, &n2.borrow().left)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let layer_one_one = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let layer_one_two = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let layer_two_one = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let layer_two_two = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let layer_two_three = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let layer_two_four = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        if let Some(node) = &layer_one_one {
            node.borrow_mut().left = layer_two_one;
            node.borrow_mut().right = layer_two_two;
        }

        if let Some(node) = &layer_one_two {
            node.borrow_mut().left = layer_two_three;
            node.borrow_mut().right = layer_two_four;
        }

        if let Some(node) = &root {
            node.borrow_mut().left = layer_one_one;
            node.borrow_mut().right = layer_one_two;
        }

        println!("{:?}", root);

        let result = is_symmetric(root);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let layer_one_one = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let layer_one_two = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let layer_two_one = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let layer_two_two = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        if let Some(node) = &layer_one_one {
            node.borrow_mut().right = layer_two_one;
        }

        if let Some(node) = &layer_one_two {
            node.borrow_mut().right = layer_two_two;
        }

        if let Some(node) = &root {
            node.borrow_mut().left = layer_one_one;
            node.borrow_mut().right = layer_one_two;
        }

        println!("{:?}", root);

        let result = is_symmetric(root);
        assert_eq!(result, false);
    }
}
