#![allow(unused)]

struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut t = t.chars().collect::<Vec<char>>();
        s.sort();
        t.sort();
        for i in 0..s.len() {
            if s[i] != t[i] {
                return t[i];
            }
        }

        t.last().unwrap().clone()
    }
    pub fn optimize_time_space_find_the_difference(s: String, t: String) -> char {
        let mut retval: u8 = 0;
        for schar in s.bytes().into_iter() {
            retval ^= schar;
        }
        for tchar in t.bytes().into_iter() {
            retval ^= tchar;
        }
        retval as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "abcd".to_string();
        let t = "abcde".to_string();
        let result = Solution::find_the_difference(s, t);
        let expected = 'e';
        assert_eq!(result, expected);
    }
    #[test]
    fn test_2() {
        let s = "".to_string();
        let t = "y".to_string();
        let result = Solution::find_the_difference(s, t);
        let expected = 'y';
        assert_eq!(result, expected);
    }
    #[test]
    fn test_2_2() {
        let s = "a".to_string();
        let t = "aa".to_string();
        let result = Solution::find_the_difference(s, t);
        let expected = 'a';
        assert_eq!(result, expected);
    }
}
