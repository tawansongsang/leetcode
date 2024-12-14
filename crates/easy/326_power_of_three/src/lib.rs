#![allow(unused)]

struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n == 0 {
            return false;
        }

        if n == 1 {
            return true;
        }

        if n % 3 != 0 {
            return false;
        }

        Self::is_power_of_three(n / 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 27;
        let result = Solution::is_power_of_three(n);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let n = 0;
        let result = Solution::is_power_of_three(n);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let n = -1;
        let result = Solution::is_power_of_three(n);
        let output = false;
        assert_eq!(result, output);
    }
}
