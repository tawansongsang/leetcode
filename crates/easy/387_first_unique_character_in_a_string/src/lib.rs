#![allow(unused)]

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<char, usize> = HashMap::new();
        s.chars().for_each(|c| {
            map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        });
        for (i, c) in s.chars().enumerate() {
            if let Some(count) = map.get(&c) {
                if count == &1 {
                    return i as i32;
                }
            }
        }
        -1
    }
    pub fn optimize_time_first_uniq_char(s: String) -> i32 {
        let mut first = i32::MAX;

        ('a'..='z').for_each(|c| {
            if s.contains(c) {
                if s.find(c) == s.rfind(c) {
                    first = first.min(s.find(c).unwrap() as i32);
                }
            }
        });

        if first == i32::MAX {
            -1
        } else {
            first
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "leetcode".to_string();
        let result = Solution::first_uniq_char(s);
        let expected = 0;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_2() {
        let s = "loveleetcode".to_string();
        let result = Solution::first_uniq_char(s);
        let expected = 2;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_3() {
        let s = "aabb".to_string();
        let result = Solution::first_uniq_char(s);
        let expected = -1;
        assert_eq!(result, expected);
    }
}
