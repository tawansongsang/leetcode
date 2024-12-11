#![allow(unused)]
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut words: HashMap<char, &str> = HashMap::new();
        let mut words_2: HashMap<&str, char> = HashMap::new();
        let s: Vec<&str> = s.split_whitespace().collect();
        if s.len() != pattern.len() {
            return false;
        }
        let mut pattern = pattern.chars();
        for i in 0..s.len() {
            if let Some(p) = pattern.next() {
                if !words.contains_key(&p) {
                    words.insert(p, s[i]);
                }

                if !words_2.contains_key(s[i]) {
                    words_2.insert(s[i], p);
                }

                if let Some(word) = words.get(&p) {
                    if *word != s[i] {
                        return false;
                    }
                }

                if let Some(word_2) = words_2.get(s[i]) {
                    if *word_2 != p {
                        return false;
                    }
                }
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
        let pattern = String::from("abba");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern(pattern, s);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat fish");
        let result = Solution::word_pattern(pattern, s);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let pattern = String::from("aaaa");
        let s = String::from("dog cat cat dog");
        let result = Solution::word_pattern(pattern, s);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_34() {
        let pattern = String::from("abba");
        let s = String::from("dog dog dog dog");
        let result = Solution::word_pattern(pattern, s);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_38() {
        let pattern = String::from("aaa");
        let s = String::from("aa aa aa aa");
        let result = Solution::word_pattern(pattern, s);
        let output = false;
        assert_eq!(result, output);
    }
}
