struct Solution;

#[allow(unused)]
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        if num_rows >= 1 {
            let row = vec![1];
            result.push(row);
        }
        if num_rows >= 2 {
            let row = vec![1, 1];
            result.push(row)
        }

        let mut n = 2;

        while n < num_rows as usize {
            let prev_row = &result[n - 1];
            let max_prev_row = prev_row.len();
            let mut new_row: Vec<i32> = vec![1];
            let mut prev_row_idx = 0;
            while prev_row_idx + 1 < max_prev_row {
                let new_val = prev_row[prev_row_idx] + prev_row[prev_row_idx + 1];
                new_row.push(new_val);
                prev_row_idx += 1;
            }
            new_row.push(1);
            result.push(new_row);

            n += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::generate(5);
        let output = vec![
            [1].to_vec(),
            [1, 1].to_vec(),
            [1, 2, 1].to_vec(),
            [1, 3, 3, 1].to_vec(),
            [1, 4, 6, 4, 1].to_vec(),
        ];
        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        let result = Solution::generate(1);
        let output = vec![[1].to_vec()];
        assert_eq!(result, output);
    }

    #[test]
    fn test_me() {
        let result = Solution::generate(2);
        let output = vec![[1].to_vec(), [1, 1].to_vec()];
        assert_eq!(result, output);
    }
}
