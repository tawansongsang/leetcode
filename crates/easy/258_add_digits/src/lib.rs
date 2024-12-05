#![allow(unused)]

struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num < 10 {
            return num;
        }

        let quotient = num / 10;
        let remainder = num % 10;

        Self::add_digits(quotient + remainder)
    }

    pub fn add_digits_optimization(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        1 + ((num - 1) % 9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = 38;
        let result = Solution::add_digits(input);
        let output = 2;
        let result_opt = Solution::add_digits_optimization(input);
        assert_eq!(result, output);
        assert_eq!(result_opt, output);
    }
    #[test]
    fn test_2() {
        let input = 0;
        let result = Solution::add_digits(input);
        let output = 0;
        let result_opt = Solution::add_digits_optimization(input);
        assert_eq!(result, output);
        assert_eq!(result_opt, output);
    }
    #[test]
    fn test_special() {
        let input = 129;
        let result = Solution::add_digits(input);
        let output = 3;
        let result_opt = Solution::add_digits_optimization(input);
        assert_eq!(result, output);
        assert_eq!(result_opt, output);
    }
}
