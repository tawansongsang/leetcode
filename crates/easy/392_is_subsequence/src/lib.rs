#![allow(unused)]

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        if s.len() > t.len() {
            return false;
        }

        let mut s: Vec<char> = s.chars().into_iter().collect();
        let mut t: Vec<char> = t.chars().into_iter().collect();

        let mut i = 0;
        let mut j = 0;

        while i < s.len() && j < t.len() {
            if s[i] != t[j] {
                j += 1;
            } else {
                i += 1;
                j += 1;
            }
        }

        if i == s.len() {
            true
        } else {
            false
        }
    }

    pub fn optimize_space_is_subsequence(s: String, t: String) -> bool {
        let mut seq = s.chars();
        let mut ch = seq.next();

        for c in t.chars() {
            if let Some(v) = ch {
                if c == v {
                    ch = seq.next();
                }
            } else {
                return true;
            }
        }

        ch.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let result = Solution::is_subsequence(s, t);
        let expected = true;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        let result = Solution::is_subsequence(s, t);
        let expected = false;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_3() {
        let s = String::from("");
        let t = String::from("ahbgdc");
        let result = Solution::is_subsequence(s, t);
        let expected = true;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_13() {
        let s = String::from("acb");
        let t = String::from("ahbgdc");
        let result = Solution::is_subsequence(s, t);
        let expected = false;
        assert_eq!(result, expected);
    }
}
