struct Solution;

#[allow(unused)]
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut result = 0;
        for i in 0..32 {
            if (n >> i & 1) == 1 {
                result += 1;
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
        let input = 11;
        let result = Solution::hamming_weight(input);
        let output = 3;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input = 128;
        let result = Solution::hamming_weight(input);
        let output = 1;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let input = 2147483645;
        let result = Solution::hamming_weight(input);
        let output = 30;
        assert_eq!(result, output);
    }
}
