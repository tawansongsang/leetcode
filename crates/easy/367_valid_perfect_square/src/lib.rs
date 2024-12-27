#![allow(unused)]

struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        use std::cmp::Ordering;
        let (mut low, mut high) = (1, num);
        while low <= high {
            let mid = low + (high - low) / 2;
            match mid.checked_mul(mid) {
                Some(square) => match square.cmp(&num) {
                    Ordering::Equal => return true,
                    Ordering::Less => low = mid + 1,
                    Ordering::Greater => high = mid - 1,
                },
                None => high = mid - 1,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let num = 16;
        let result = Solution::is_perfect_square(num);
        let expected = true;
        assert_eq!(result, expected);
    }
    #[test]
    fn test_2() {
        let num = 14;
        let result = Solution::is_perfect_square(num);
        let expected = false;
        assert_eq!(result, expected);
    }
}
