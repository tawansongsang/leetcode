struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let mut n = n;
        while n > 0 {
            if n == 1 {
                return true;
            } else if n % 2 == 0 {
                n = n / 2;
            } else {
                return false;
            }
        }
        false
    }

    pub fn is_power_of_two_2(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = 1;
        let result = Solution::is_power_of_two(input);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input = 16;
        let result = Solution::is_power_of_two(input);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let input = 3;
        let result = Solution::is_power_of_two(input);
        let output = false;
        assert_eq!(result, output);
    }
}
