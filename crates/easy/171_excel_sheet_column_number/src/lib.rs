use std::collections::HashMap;

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut chars: HashMap<&str, i32> = HashMap::from([
            ("A", 1),
            ("B", 2),
            ("C", 3),
            ("D", 4),
            ("E", 5),
            ("F", 6),
            ("G", 7),
            ("H", 8),
            ("I", 9),
            ("J", 10),
            ("K", 11),
            ("L", 12),
            ("M", 13),
            ("N", 14),
            ("O", 15),
            ("P", 16),
            ("Q", 17),
            ("R", 18),
            ("S", 19),
            ("T", 20),
            ("U", 21),
            ("V", 22),
            ("W", 23),
            ("X", 24),
            ("Y", 25),
            ("Z", 26),
        ]);

        let mut result = 0;

        for c in column_title.chars() {
            let c = c.to_string();
            let n = chars.get(c.as_str()).unwrap();
            result = result * 26 + n;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = String::from("A");
        let result = Solution::title_to_number(input);
        let output = 1;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input = String::from("AB");
        let result = Solution::title_to_number(input);
        let output = 28;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let input = String::from("ZY");
        let result = Solution::title_to_number(input);
        let output = 701;
        assert_eq!(result, output);
    }

    #[test]
    fn test_4() {
        let input = String::from("FXSHRXW");
        let result = Solution::title_to_number(input);
        let output = 2147483647;
        assert_eq!(result, output);
    }
}
