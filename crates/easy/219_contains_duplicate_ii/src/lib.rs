struct Solution;

#[allow(unused)]
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..nums.len() {
                if j - i > k {
                    break;
                }
                if nums[i] == nums[j] {
                    return true;
                }
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
        let input = vec![1, 2, 3, 1];
        let k = 3;
        let result = Solution::contains_nearby_duplicate(input, k);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_2() {
        let input = vec![1, 0, 1, 1];
        let k = 1;
        let result = Solution::contains_nearby_duplicate(input, k);
        let output = true;
        assert_eq!(result, output);
    }
    #[test]
    fn test_3() {
        let input = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        let result = Solution::contains_nearby_duplicate(input, k);
        let output = false;
        assert_eq!(result, output);
    }
}
