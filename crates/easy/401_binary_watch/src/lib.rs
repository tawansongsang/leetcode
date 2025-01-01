#![allow(unused)]

struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;
        let mut result: Vec<String> = Vec::new();
        for i in 0i32..12 {
            for j in 0i32..60 {
                if i.count_ones() + j.count_ones() == turned_on {
                    result.push(format!("{i}:{j:02}"))
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let turned_on = 1;
        let result = Solution::read_binary_watch(turned_on);
        let expected = vec![
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
        ];
        assert_eq!(result, expected);
    }
    #[test]
    fn test_2() {
        let turned_on = 9;
        let result = Solution::read_binary_watch(turned_on);
        let expected: Vec<String> = Vec::new();
        assert_eq!(result, expected);
    }
}
