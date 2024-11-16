struct Solution;

#[allow(unused)]
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        const CHARS: [&str; 26] = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
        ];
        let mut column_number = column_number as usize;
        let mut result: Vec<&str> = Vec::new();
        while column_number > 0 {
            column_number -= 1;
            let remainder = column_number % 26;
            result.push(CHARS[remainder]);
            column_number /= 26;
        }

        result.reverse();
        result.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::convert_to_title(1);
        assert_eq!(result, "A");
    }

    #[test]
    fn test_2() {
        let result = Solution::convert_to_title(28);
        assert_eq!(result, "AB");
    }
    #[test]
    fn test_3() {
        let result = Solution::convert_to_title(701);
        assert_eq!(result, "ZY");
    }
}
