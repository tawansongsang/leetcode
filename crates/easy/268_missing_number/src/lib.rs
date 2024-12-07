#![allow(unused)]

struct Solution;

impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        if let Some(first) = nums.first() {
            if first == &1 {
                return 0;
            }
        }

        for i in 0..(nums.len() - 1) {
            if nums[i + 1] - nums[i] != 1 {
                return nums[i] + 1;
            }
        }

        nums.last().unwrap_or(&0) + 1
    }

    pub fn missing_number_optimal(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        ((nums.len() * (nums.len() +1))/2) as i32 - sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![3, 0, 1];
        let result = Solution::missing_number(input);
        let output = 2;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input = vec![0, 1];
        let result = Solution::missing_number(input);
        let output = 2;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let input = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let result = Solution::missing_number(input);
        let output = 8;
        assert_eq!(result, output);
    }
    #[test]
    fn test_110() {
        let input = vec![1];
        let result = Solution::missing_number(input);
        let output = 0;
        assert_eq!(result, output);
    }
}
