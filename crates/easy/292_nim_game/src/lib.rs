#![allow(unused)]

struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        let result = Solution::can_win_nim(n);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let n = 1;
        let result = Solution::can_win_nim(n);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let n = 2;
        let result = Solution::can_win_nim(n);
        let output = true;
        assert_eq!(result, output);
    }
}
