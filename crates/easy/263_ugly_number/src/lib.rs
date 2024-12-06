#![allow(unused)]

struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 1 {
            return true;
        } else if n == 0 {
            return false;
        }

        let mut n = n;
        while n != 1 {
            if n % 2 == 0 {
                n = n / 2;
            } else if n % 3 == 0 {
                n = n / 3;
            } else if n % 5 == 0 {
                n = n / 5;
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 6;
        let result = Solution::is_ugly(n);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let n = 1;
        let result = Solution::is_ugly(n);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let n = 14;
        let result = Solution::is_ugly(n);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_1003() {
        let n = -2147483648;
        let result = Solution::is_ugly(n);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_1012() {
        let n = 0;
        let result = Solution::is_ugly(n);
        let output = false;
        assert_eq!(result, output);
    }
}
