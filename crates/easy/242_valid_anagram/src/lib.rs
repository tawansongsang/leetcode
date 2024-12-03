use std::collections::HashMap;

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut original_char: HashMap<&u8, usize> = HashMap::new();
        s.as_bytes().into_iter().for_each(|c| {
            original_char
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });

        let mut anagram_char: HashMap<&u8, usize> = HashMap::new();
        t.as_bytes().into_iter().for_each(|c| {
            anagram_char
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });

        original_char == anagram_char
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        let result = Solution::is_anagram(s, t);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let s = String::from("rat");
        let t = String::from("car");
        let result = Solution::is_anagram(s, t);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_36() {
        let s = String::from("a");
        let t = String::from("ab");
        let result = Solution::is_anagram(s, t);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_41() {
        let s = String::from("ab");
        let t = String::from("a");
        let result = Solution::is_anagram(s, t);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_42() {
        let s = String::from("aa");
        let t = String::from("a");
        let result = Solution::is_anagram(s, t);
        let output = false;
        assert_eq!(result, output);
    }
}
