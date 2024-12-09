#![allow(unused)]

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            if nums[r] != 0 {
                let tmp = nums[l];
                nums[l] = nums[r];
                nums[r] = tmp;
                l += 1;
            }

            r += 1
        }

        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        let output = vec![1, 3, 12, 0, 0];
        assert_eq!(nums, output);
    }
    #[test]
    fn test_2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        let output = vec![0];
        assert_eq!(nums, output);
    }
    #[test]
    fn test_27() {
        let mut nums = vec![1, 0, 1];
        Solution::move_zeroes(&mut nums);
        let output = vec![1, 1, 0];
        assert_eq!(nums, output);
    }
}
