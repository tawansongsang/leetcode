#![allow(unused)]

struct NumArray {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let mut sums = 0;
        let left = left as usize;
        let right = right as usize;
        for i in left..(right + 1) {
            sums += self.nums[i];
        }

        sums
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        let left = 0;
        let right = 2;
        let result = nums.sum_range(left, right);
        let output = 1;
        assert_eq!(result, output);
        let left = 2;
        let right = 5;
        let result = nums.sum_range(left, right);
        let output = -1;
        assert_eq!(result, output);
        let left = 0;
        let right = 5;
        let result = nums.sum_range(left, right);
        let output = -3;
        assert_eq!(result, output);
    }
}

// Best Time Complexity
// struct NumArray {
//     prefix_sum_array: Vec<i32>,
// }

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl NumArray {

//     fn new(nums: Vec<i32>) -> Self {
//         NumArray{
//             prefix_sum_array: nums.iter().fold(vec![], |mut acc, x| {
//                 acc.push(acc.last().unwrap_or_else(|| &0) + x) ;
//                 acc
//         }),
//         }
//     }

//     fn sum_range(&self, left: i32, right: i32) -> i32 {
//         if left == 0 {
//             self.prefix_sum_array[right as usize]
//         } else {
//             self.prefix_sum_array[right as usize] - self.prefix_sum_array[left as usize - 1]
//         }
//     }
// }
