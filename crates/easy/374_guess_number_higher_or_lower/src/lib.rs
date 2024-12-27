#![allow(unused)]
#![allow(non_snake_case)]

struct Solution;

unsafe fn guess(num: i32) -> i32 {
    let pick = 6;
    if num < pick {
        1
    } else if num > pick {
        -1
    } else {
        0
    }
}

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        loop {
            let mid = low + (high - low) / 2;
            match guess(mid) {
                0 => return mid,
                -1 => high = mid - 1,
                _ => low = mid + 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 10;
        let result = unsafe { Solution::guessNumber(n) };
        let expected = 6;
        assert_eq!(result, expected);
    }
}
