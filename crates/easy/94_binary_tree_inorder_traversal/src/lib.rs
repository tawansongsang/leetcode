use std::cell::RefCell;
use std::rc::Rc;
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut traversal_nodes: Vec<i32> = Vec::new();

    inner_inorder_traversal(&root, &mut traversal_nodes);

    traversal_nodes
}

pub fn inner_inorder_traversal(
    node: &Option<Rc<RefCell<TreeNode>>>,
    traversal_nodes: &mut Vec<i32>,
) {
    if let Some(n) = node {
        let branch = n.borrow();
        inner_inorder_traversal(&branch.left, traversal_nodes);
        traversal_nodes.push(branch.val);
        inner_inorder_traversal(&branch.right, traversal_nodes);
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let layer_one = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let layer_two = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        if let Some(node) = &layer_one {
            node.borrow_mut().left = layer_two;
        }
        if let Some(node) = &root {
            node.borrow_mut().right = layer_one;
        }

        println!("{:?}", root);

        let result = inorder_traversal(root);
        let output = vec![1, 3, 2];
        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        let root = None;
        let result = inorder_traversal(root);
        let output = Vec::new();
        assert_eq!(result, output);
    }

    #[test]
    fn test_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = inorder_traversal(root);
        let output = vec![1];
        assert_eq!(result, output);
    }
}
