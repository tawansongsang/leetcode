use std::collections::HashMap;

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut num_counts: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            num_counts
                .entry(num)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }

        let results = num_counts
            .iter()
            .filter(|num| num.1.eq(&1))
            .collect::<Vec<_>>();

        *results[0].0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 2, 1];
        let result = Solution::single_number(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 1, 2, 1, 2];
        let result = Solution::single_number(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        let result = Solution::single_number(nums);
        assert_eq!(result, 1);
    }
}
