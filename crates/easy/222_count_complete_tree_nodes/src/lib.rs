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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                Self::count_nodes(node.borrow().left.clone())
                    + Self::count_nodes(node.borrow().right.clone())
                    + 1
            }
            None => 0,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let six = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        })));
        let five = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })));
        let four = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));
        let three = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: six,
            right: None,
        })));
        let two = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: four,
            right: five,
        })));
        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: two,
            right: three,
        })));
        let result = Solution::count_nodes(input);
        let output = 6;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input = None;
        let result = Solution::count_nodes(input);
        let output = 0;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let result = Solution::count_nodes(input);
        let output = 1;
        assert_eq!(result, output);
    }
}
