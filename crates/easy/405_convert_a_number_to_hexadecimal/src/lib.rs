#![allow(unused)]

struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        let mut num = num as u32;
        let hexs = [
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
        ];
        let mut result = String::new();

        if num == 0 {
            return "0".to_string();
        }

        while num != 0 {
            let idx = (num % 16) as usize;
            result.push_str(hexs[idx]);
            num /= 16;
        }

        result.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 26;
        let result = Solution::to_hex(num);
        let expected = "1a".to_string();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_2() {
        let num = -1;
        let result = Solution::to_hex(num);
        let expected = "ffffffff".to_string();
        assert_eq!(result, expected);
    }
}
