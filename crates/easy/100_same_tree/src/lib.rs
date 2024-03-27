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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (Some(ref m), Some(ref n)) => {
            let m = m.borrow();
            let n = n.borrow();
            m.val == n.val
                && is_same_tree(n.left.clone(), m.left.clone())
                && is_same_tree(n.right.clone(), m.right.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root_one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        if let Some(node) = &root_one {
            node.borrow_mut().left = left;
            node.borrow_mut().right = right;
        }

        println!("{:?}", root_one);

        let root_two = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        if let Some(node) = &root_two {
            node.borrow_mut().left = left;
            node.borrow_mut().right = right;
        }

        println!("{:?}", root_two);

        let result = is_same_tree(root_one, root_two);
        let output = true;
        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        let root_one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        if let Some(node) = &root_one {
            node.borrow_mut().left = left;
        }

        println!("{:?}", root_one);

        let root_two = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        if let Some(node) = &root_two {
            node.borrow_mut().right = right;
        }

        println!("{:?}", root_two);

        let result = is_same_tree(root_one, root_two);
        let output = false;
        assert_eq!(result, output);
    }

    #[test]
    fn test_3() {
        let root_one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        if let Some(node) = &root_one {
            node.borrow_mut().left = left;
            node.borrow_mut().right = right;
        }

        println!("{:?}", root_one);

        let root_two = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        if let Some(node) = &root_two {
            node.borrow_mut().left = left;
            node.borrow_mut().right = right;
        }

        println!("{:?}", root_two);

        let result = is_same_tree(root_one, root_two);
        let output = false;
        assert_eq!(result, output);
    }
}
