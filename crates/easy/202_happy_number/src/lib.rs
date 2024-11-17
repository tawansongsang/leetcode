use std::collections::HashSet;

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut n = n;
        while n != 1 && !seen.contains(&n) {
            seen.insert(n);
            n = Self::get_next(n);
            if seen.contains(&n) {
                return false;
            }
        }

        n == 1
    }

    fn get_next(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit.pow(2);
            n /= 10;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = 19;
        let result = Solution::is_happy(input);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input = 2;
        let result = Solution::is_happy(input);
        let output = false;
        assert_eq!(result, output);
    }
}
