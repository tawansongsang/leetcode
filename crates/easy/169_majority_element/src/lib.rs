struct Solution;

#[allow(unused)]
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;
        for num in nums {
            if count == 0 {
                candidate = num;
            }

            if num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }
        
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![3, 2, 3];
        let result = Solution::majority_element(input);
        let output = 3;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input = vec![2, 2, 1, 1, 1, 2, 2];
        let result = Solution::majority_element(input);
        let output = 2;
        assert_eq!(result, output);
    }
}
