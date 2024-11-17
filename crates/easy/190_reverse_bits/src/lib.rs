struct Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        x.reverse_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // 00000010100101000001111010011100
        let input = 43261596;
        let result = Solution::reverse_bits(input);
        // 00111001011110000010100101000000
        let output = 964176192;
        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        // 11111111111111111111111111111101
        let input = 4294967293;
        let result = Solution::reverse_bits(input);
        // 10111111111111111111111111111111
        let output = 3221225471;
        assert_eq!(result, output);
    }
}
