#![allow(unused)]

struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut result = Vec::new();
        for num in nums1 {
            *map.entry(num).or_insert(0) += 1;
        }
        for num in nums2 {
            if let Some(count) = map.get_mut(&num) {
                if *count > 0 {
                    result.push(num);
                    *count -= 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        let result = Solution::intersect(nums1, nums2);
        let output = vec![2, 2];
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = Solution::intersect(nums1, nums2);
        let output = vec![4, 9];
        assert_eq!(result, output);
    }
}
