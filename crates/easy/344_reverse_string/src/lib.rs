#![allow(unused)]

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let max_range = s.len();
        let mid = max_range / 2;
        for i in 0..mid {
            let tmp = s[i];
            let swap_i = max_range - i - 1;
            s[i] = s[swap_i];
            s[swap_i] = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        let result = Solution::reverse_string(&mut s);
        let output = vec!['o', 'l', 'l', 'e', 'h'];
        assert_eq!(s, output);
    }
    #[test]
    fn test_2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let result = Solution::reverse_string(&mut s);
        let output = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        assert_eq!(s, output);
    }
}
