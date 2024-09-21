use std::{cell::RefCell, rc::Rc};

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

pub fn dfs(nums: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if start > end {
        return None;
    }

    let mid = (start + end) / 2;
    let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
    if mid == 0 {
        root.borrow_mut().left = None;
    } else {
        root.borrow_mut().left = dfs(nums, start, mid - 1);
    }
    root.borrow_mut().right = dfs(nums, mid + 1, end);

    Some(root)
}

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let end = nums.len() - 1;
    dfs(&nums, 0, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let result = sorted_array_to_bst(nums);
        let root = Rc::new(RefCell::new(TreeNode::new(0)));
        let l_one_r = Rc::new(RefCell::new(TreeNode::new(5)));
        let l_one_l = Rc::new(RefCell::new(TreeNode::new(-10)));
        let l_two_rr = Rc::new(RefCell::new(TreeNode::new(9)));
        let l_two_lr = Rc::new(RefCell::new(TreeNode::new(-3)));
        l_one_l.borrow_mut().right = Some(l_two_lr);
        l_one_l.borrow_mut().left = None;
        l_one_r.borrow_mut().right = Some(l_two_rr);
        l_one_r.borrow_mut().left = None;
        root.borrow_mut().right = Some(l_one_r);
        root.borrow_mut().left = Some(l_one_l);

        assert_eq!(result, Some(root));
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3];
        let result = sorted_array_to_bst(nums);
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let l_one = Rc::new(RefCell::new(TreeNode::new(3)));
        root.borrow_mut().right = Some(l_one);
        root.borrow_mut().left = None;
        // let root = Rc::new(RefCell::new(TreeNode::new(3)));
        // let l_one = Rc::new(RefCell::new(TreeNode::new(1)));
        // root.borrow_mut().right = None;
        // root.borrow_mut().left = Some(l_one);

        assert_eq!(result, Some(root));
    }
}
