struct Solution;

#[allow(unused)]
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut ret: Vec<i32> = Vec::new();
        for i in 0..row_index + 1 {
            if i == 0 || i == 1 {
                ret.push(1);
                continue;
            }

            for j in 1..i {
                for k in 0..j {
                    if j % 2 == 0 {
                        if k % 2 == 0 {
                            ret[j] -= ret[k];
                        } else {
                            ret[j] += ret[k];
                        }
                    } else {
                        if k % 2 == 0 {
                            ret[j] += ret[k];
                        } else {
                            ret[j] -= ret[k];
                        }
                    }
                }
            }

            ret.push(1);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::get_row(3);
        let output = vec![1, 3, 3, 1];
        assert_eq!(result, output);
    }

    #[test]
    fn test_2() {
        let result = Solution::get_row(0);
        let output = vec![1];
        assert_eq!(result, output);
    }

    #[test]
    fn test_3() {
        let result = Solution::get_row(1);
        let output = vec![1, 1];
        assert_eq!(result, output);
    }
}
