use std::collections::HashSet;

struct Solution;

#[allow(unused)]
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set_nums: HashSet<i32> = HashSet::new();

        for num in nums {
            if set_nums.contains(&num) {
                return true;
            }

            set_nums.insert(num);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![1, 2, 3, 1];
        let result = Solution::contains_duplicate(input);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input = vec![1, 2, 3, 4];
        let result = Solution::contains_duplicate(input);
        let output = false;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let input = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let result = Solution::contains_duplicate(input);
        let output = true;
        assert_eq!(result, output);
    }
}
