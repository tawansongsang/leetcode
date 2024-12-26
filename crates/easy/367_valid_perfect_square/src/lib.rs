#![allow(unused)]

struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut left = 1;
        let mut right = num;
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_squared = mid * mid;
            if mid_squared == num {
                return true;
            } else if mid_squared < num {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 16;
        let result = Solution::is_perfect_square(num);
        let expected = true;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_2() {
        let num = 14;
        let result = Solution::is_perfect_square(num);
        let expected = false;
        assert_eq!(result, expected);
    }
}
