#![allow(unused)]

struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity((n + 1) as usize);
        for i in 0..(n + 1) {
            let count_ones = i.count_ones() as i32;
            ans.push(count_ones);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 2;
        let result = Solution::count_bits(n);
        let output: Vec<i32> = vec![0, 1, 1];
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let n = 5;
        let result = Solution::count_bits(n);
        let output: Vec<i32> = vec![0, 1, 1, 2, 1, 2];
        assert_eq!(result, output);
    }
}
