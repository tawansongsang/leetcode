#![allow(unused)]

struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let mut n = n;
        let mut remainder = i32::default();
        loop {
            if n == 0 {
                return false;
            }

            if n == 1 {
                return true;
            }
            remainder = n % 4;

            if remainder != 0 {
                return false;
            }

            n = n / 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 16;
        let result = Solution::is_power_of_four(n);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let n = 5;
        let result = Solution::is_power_of_four(n);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let n = 1;
        let result = Solution::is_power_of_four(n);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_973() {
        let n = 17;
        let result = Solution::is_power_of_four(n);
        let output = false;
        assert_eq!(result, output);
    }
}
