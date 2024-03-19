pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            let num_i = nums[i];
            let num_j = nums[j];
            let sum = num_i + num_j;
            if sum == target {
                output.push(i as i32);
                output.push(j as i32);
                break;
            }
        }
        if output.len() > 1 {
            break;
        }
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        let output = vec![0, 1];
        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum(nums, target);
        let output = vec![1, 2];
        assert_eq!(result, output);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum(nums, target);
        let output = vec![0, 1];
        assert_eq!(result, output);
    }
}
