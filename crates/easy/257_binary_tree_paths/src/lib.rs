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

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut order = Vec::new();
        Self::dfs(&root, "", &mut order);
        order
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, path: &str, order: &mut Vec<String>) {
        if let Some(n) = node {
            let mut new_path = format!("{path}{}", n.borrow().val);
            let left_node = &n.borrow().left;
            let right_node = &n.borrow().right;
            if left_node.is_none() && right_node.is_none() {
                order.push(new_path);
            } else {
                new_path.push_str("->");
                Self::dfs(left_node, &new_path, order);
                Self::dfs(right_node, &new_path, order);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let five = Rc::new(RefCell::new(TreeNode::new(5)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        two.borrow_mut().right = Some(five);
        let root = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(two),
            right: Some(three),
        }));
        let input = Some(root);
        let result = Solution::binary_tree_paths(input);
        let output = vec!["1->2->5", "1->3"];
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let root = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }));
        let input = Some(root);
        let result = Solution::binary_tree_paths(input);
        let output = vec!["1"];
        assert_eq!(result, output);
    }
}
