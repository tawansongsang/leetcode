#![allow(unused)]
struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut set1 = nums1.iter().collect::<std::collections::HashSet<_>>();
        let mut set2 = nums2.iter().collect::<std::collections::HashSet<_>>();
        for num in set1.intersection(&set2) {
            result.push(**num);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_1() {
        let num1 = vec![1, 2, 2, 1];
        let num2 = vec![2, 2];
        let result = Solution::intersection(num1, num2);
        let output = vec![2];
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let num1 = vec![4, 9, 5];
        let num2 = vec![9, 4, 9, 8, 4];
        let result = Solution::intersection(num1, num2);
        let output = vec![9, 4];
        assert_eq!(result, output);
    }
}
